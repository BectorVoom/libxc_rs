//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1271/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1271<F: Float>(t15573: F, t29160: F, t7772: F, t2822: F, t28916: F, t100275: F, t1250: F, t251: F, t70071: F, t100090: F, t100268: F, t7775: F, t7788: F, t7796: F, t93158: F, t96273: F, t97377: F, t97385: F) -> (F, F, F) {
    let t100784 = t15573 * t29160;
    let t100785 = t7772 * t100784;
    let t100790 = t2822 * t28916;
    let t100794 = t7772 * t100275;
    let t100799 = t70071 * t251 * t1250;
    let t100802 = -F::new(0.30918233506944444445e-4) * t100785 - F::new(0.34752604166666666667e-3) * t7788 * t100090 - F::new(0.77382407407407407407e-3) * t96273 + t93158 + t97377 - F::new(0.15476481481481481481e-2) * t100790 + F::new(0.34752604166666666667e-3) * t100268 * t7796 + F::new(0.15459116753472222222e-4) * t100794 + F::new(0.34752604166666666667e-3) * t100268 * t7775 - t97385 + F::new(0.46377350260416666667e-4) * t100799 * t7775;
    (t100784, t100790, t100802)
}
