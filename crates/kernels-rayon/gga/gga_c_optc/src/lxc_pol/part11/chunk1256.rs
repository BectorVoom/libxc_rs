//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1256/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1256(t4565: f64, t4768: f64, t4961: f64, t2669: f64, t1235: f64, t49249: f64, t13670: f64, t10991: f64, t14293: f64, t2678: f64, t2679: f64, t2722: f64, t2813: f64, t322: f64, t3835: f64, t3836: f64, t40356: f64, t4942: f64, t49882: f64, t56718: f64, t56722: f64, t56727: f64, t56732: f64, t56735: f64, t7491: f64, t8134: f64, t8209: f64, t862: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56740 = t4768 * t4565;
    let t56744 = t4961 * t4961;
    let t56745 = t2669 * t56744;
    let t56752 = t49249 * t1235;
    let t56756 = t13670 * t4565;
    let t56764 = -7.0_f64 / 54.0_f64 * t862 * t322 * t56718 + 0.47333755318775392234e0_f64 * t10991 * t14293 * t56722 + 0.61048523203065534458e2_f64 * t7491 * t40356 * t56727 + 0.18110753103726578864e-2_f64 * t893 * t56732 - t862 * t2722 * t56735 / 36.0_f64 + 0.31555836879183594822e0_f64 * t49882 + 0.65198711173415683908e-1_f64 * t3835 * t2813 * t56740 - 0.13735917720689745254e2_f64 * t2678 * t56745 * t2679 - 0.14488602482981263091e-1_f64 * t3835 * t2813 * t56735 + 0.12073835402484385909e-1_f64 * t3835 * t3836 * t56752 - 0.10866451862235947318e0_f64 * t3835 * t3836 * t56756 + 0.63777043459628018514e5_f64 * t8134 * t4942 * t8209 * t4961;
    (t56740, t56744, t56745, t56752, t56756, t56764)
}
