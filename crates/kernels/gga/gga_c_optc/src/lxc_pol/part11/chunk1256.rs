//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1256/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1256<F: Float>(t4565: F, t4768: F, t4961: F, t2669: F, t1235: F, t49249: F, t13670: F, t10991: F, t14293: F, t2678: F, t2679: F, t2722: F, t2813: F, t322: F, t3835: F, t3836: F, t40356: F, t4942: F, t49882: F, t56718: F, t56722: F, t56727: F, t56732: F, t56735: F, t7491: F, t8134: F, t8209: F, t862: F, t893: F) -> (F, F, F, F, F, F) {
    let t56740 = t4768 * t4565;
    let t56744 = t4961 * t4961;
    let t56745 = t2669 * t56744;
    let t56752 = t49249 * t1235;
    let t56756 = t13670 * t4565;
    let t56764 = -F::new(7.0) / F::new(54.0) * t862 * t322 * t56718 + F::new(0.47333755318775392234e0) * t10991 * t14293 * t56722 + F::new(0.61048523203065534458e2) * t7491 * t40356 * t56727 + F::new(0.18110753103726578864e-2) * t893 * t56732 - t862 * t2722 * t56735 / F::new(36.0) + F::new(0.31555836879183594822e0) * t49882 + F::new(0.65198711173415683908e-1) * t3835 * t2813 * t56740 - F::new(0.13735917720689745254e2) * t2678 * t56745 * t2679 - F::new(0.14488602482981263091e-1) * t3835 * t2813 * t56735 + F::new(0.12073835402484385909e-1) * t3835 * t3836 * t56752 - F::new(0.10866451862235947318e0) * t3835 * t3836 * t56756 + F::new(0.63777043459628018514e5) * t8134 * t4942 * t8209 * t4961;
    (t56740, t56744, t56745, t56752, t56756, t56764)
}
