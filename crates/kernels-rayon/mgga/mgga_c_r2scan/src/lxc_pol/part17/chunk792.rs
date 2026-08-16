//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 792/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk792(t2483: f64, t697: f64, t1721: f64, t898: f64, t5393: f64, t5: f64, t736: f64, t1754: f64, t2788: f64, t2782: f64, t584: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7730 = 0.1301229756036208781e0_f64 * t2483 * t697;
    let t7737 = t898 * t1721;
    let t7739 = 48.0_f64 * t5393;
    let t7741 = t2483 * t5;
    let t7743 = 0.10843581300301739842e-1_f64 * t7741 * t736;
    let t7745 = t2788 * t1754;
    let t7751 = 0.1143056e0_f64 * t584 * t2782 * t591;
    (t7730, t7737, t7739, t7743, t7745, t7751)
}
