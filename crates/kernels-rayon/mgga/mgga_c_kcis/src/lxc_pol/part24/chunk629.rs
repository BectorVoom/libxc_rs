//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 629/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk629(t3508: f64, t6272: f64, t3507: f64, t1662: f64, t1851: f64, t3515: f64, t3520: f64, t1252: f64, t1253: f64, t6276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6758 = t3508 * t6272;
    let t6759 = t3507 * t6758;
    let t6762 = t1662 * t1851;
    let t6763 = t3515 * t6762;
    let t6766 = t3520 * t6272;
    let t6767 = t1252 * t6766;
    let t6770 = t1253 * t6276;
    let t6771 = t1252 * t6770;
    let t6774 = t1851 * t1851;
    (t6758, t6759, t6762, t6763, t6766, t6767, t6770, t6771, t6774)
}
