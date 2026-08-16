//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1522/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1522(t13654: f64, t913: f64, t893: f64, t2929: f64, t4471: f64, t4497: f64, t959: f64, t2904: f64, t952: f64, t3216: f64, t4696: f64, t13550: f64) -> (f64, f64, f64, f64, f64) {
    let t13655 = t13654 * t913;
    let t13657 = 1.0_f64 * t893 * t13655;
    let t13658 = t2929 * t4471;
    let t13659 = t13658 * t4497;
    let t13661 = 0.34631718211362927518e2_f64 * t959 * t13659;
    let t13662 = t2904 * t4471;
    let t13663 = t13662 * t952;
    let t13665 = 0.23392894490538584828e1_f64 * t959 * t13663;
    let t13666 = t4696 * t3216;
    let t13675 = 0.22076e0_f64 * t13550;
    (t13657, t13661, t13665, t13666, t13675)
}
