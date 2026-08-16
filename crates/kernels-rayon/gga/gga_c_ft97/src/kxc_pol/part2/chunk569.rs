//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 569/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk569(t200: f64, t3750: f64, t680: f64, t2379: f64, t3733: f64, t202: f64, t222: f64, t237: f64, t1113: f64, t25: f64, t679: f64, t688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3751 = t3750 * t200;
    let t3752 = t680 * t3751;
    let t3755 = t2379 * t3733;
    let t3758 = t202 * t222;
    let t3759 = t3758 * t237;
    let t3760 = t1113 * t25;
    let t3761 = t679 * t688;
    (t3751, t3752, t3755, t3758, t3759, t3760, t3761)
}
