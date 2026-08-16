//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 922/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk922(t2766: f64, t6353: f64, t10491: f64, t1508: f64, t10478: f64, t25188: f64, t848: f64, t2770: f64, t7091: f64, t2842: f64, t6260: f64, t309: f64, t43524: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112663 = t2766 * t6353;
    let t112680 = t10491 * t1508;
    let t112746 = t10478 * t1508;
    let t112760 = t848 * t25188;
    let t112790 = t2770 * t7091;
    let t112883 = t2842 * t6260;
    let t112888 = t43524 * t309;
    (t112663, t112680, t112746, t112760, t112790, t112883, t112888)
}
