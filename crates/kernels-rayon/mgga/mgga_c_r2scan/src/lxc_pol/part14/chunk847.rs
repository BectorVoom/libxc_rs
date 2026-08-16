//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 847/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk847(t1862: f64, t7657: f64, t2743: f64, t5322: f64, t1838: f64, t963: f64, t1810: f64, t2798: f64, t584: f64, t1759: f64, t1748: f64, t2788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7659 = 0.2701041328e0_f64 * t7657 * t1862;
    let t7661 = 0.2701041328e0_f64 * t2743 * t5322;
    let t7662 = t963 * t1838;
    let t7664 = t963 * t1810;
    let t7666 = t584 * t2798;
    let t7667 = t7666 * t1759;
    let t7669 = t2788 * t1748;
    (t7659, t7661, t7662, t7664, t7667, t7669)
}
