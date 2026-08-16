//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 543/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk543(t1613: f64, t5551: f64, t5555: f64, t1609: f64, t58: f64, t1293: f64, t1710: f64, t6: f64, t8051: f64, t8: f64, t3076: f64, t12: f64, t391: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t22737 = t1613 * t5551;
    let t22738 = t22737 * t5555;
    let t22742 = t1609 * sigma0;
    let t22743 = t22742 * t58;
    let t22755 = t1710 * t1293;
    let t22759 = t8051 * t6;
    let t22760 = t22759 * t8;
    let t22761 = t3076 * t22760;
    let t22766 = t12 * t391;
    (t22738, t22743, t22755, t22761, t22766)
}
