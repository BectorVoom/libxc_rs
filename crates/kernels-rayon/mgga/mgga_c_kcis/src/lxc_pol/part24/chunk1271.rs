//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1271/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1271(t15573: f64, t29160: f64, t7772: f64, t2822: f64, t28916: f64, t100275: f64, t1250: f64, t251: f64, t70071: f64, t100090: f64, t100268: f64, t7775: f64, t7788: f64, t7796: f64, t93158: f64, t96273: f64, t97377: f64, t97385: f64) -> (f64, f64, f64) {
    let t100784 = t15573 * t29160;
    let t100785 = t7772 * t100784;
    let t100790 = t2822 * t28916;
    let t100794 = t7772 * t100275;
    let t100799 = t70071 * t251 * t1250;
    let t100802 = -0.30918233506944444445e-4_f64 * t100785 - 0.34752604166666666667e-3_f64 * t7788 * t100090 - 0.77382407407407407407e-3_f64 * t96273 + t93158 + t97377 - 0.15476481481481481481e-2_f64 * t100790 + 0.34752604166666666667e-3_f64 * t100268 * t7796 + 0.15459116753472222222e-4_f64 * t100794 + 0.34752604166666666667e-3_f64 * t100268 * t7775 - t97385 + 0.46377350260416666667e-4_f64 * t100799 * t7775;
    (t100784, t100790, t100802)
}
