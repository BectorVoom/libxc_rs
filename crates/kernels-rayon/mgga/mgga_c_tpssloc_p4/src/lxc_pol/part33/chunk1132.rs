//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1132/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1132(t1015: f64, t1615: f64, t344: f64, t7573: f64, t6740: f64, t2770: f64, t381: f64, t23384: f64, t7566: f64, t1054: f64, t1634: f64, t225: f64, t7594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25658 = t1015 * t1615;
    let t25682 = t7573 * t344;
    let t25683 = t6740 * t25682;
    let t25721 = t381 * t2770;
    let t25736 = t23384 * t7566;
    let t25749 = t1054 * t1634;
    let t25755 = t7594 * t225;
    (t25658, t25682, t25683, t25721, t25736, t25749, t25755)
}
