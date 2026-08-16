//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 747/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk747(t1710: f64, t4474: f64, t8051: f64, t15648: f64, t534: f64, t25: f64, t3066: f64, t4491: f64, t4455: f64, t458: f64, t4417: f64, t7763: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15712 = t1710 * t4474;
    let t15716 = t8051 * t4474;
    let t15720 = t534 * t15648;
    let t15723 = t4474 * t25;
    let t15724 = t15723 * t3066;
    let t15727 = t1710 * t4491;
    let t15734 = t458 * t4455;
    let t15736 = t7763 * t4417;
    (t15712, t15716, t15720, t15724, t15727, t15734, t15736)
}
