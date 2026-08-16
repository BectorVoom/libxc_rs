//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 948/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk948(t2289: f64, t9087: f64, t2412: f64, t8592: f64, t2410: f64, t3350: f64, t8515: f64, t8519: f64, t39277: f64, t8668: f64, t8831: f64, t35242: f64, t35246: f64, t35256: f64, t45716: f64, t45722: f64, t45724: f64, t45728: f64, t45732: f64, t45734: f64, t45736: f64, t45738: f64, t45742: f64) -> f64 {
    let t45744 = t9087 * t2289;
    let t45746 = t2412 * t8592;
    let t45750 = t2410 * t8515 * t3350 * t8519;
    let t45752 = t39277 * t8668;
    let t45754 = t39277 * t8831;
    let t45756 = -0.5124043883133942371e-4_f64 * t45716 + 0.30487649791575028314e-3_f64 * t35242 - 0.43368970657079495312e-4_f64 * t35246 + 0.40911992481368012592e-1_f64 * t45722 - 0.81823984962736025184e-1_f64 * t45724 - 0.81823984962736025184e-1_f64 * t45728 - 0.81823984962736025184e-1_f64 * t45732 + 0.25538759935978703638e-4_f64 * t45734 + 0.25538759935978703638e-4_f64 * t45736 + 0.12769379967989351819e-4_f64 * t45738 - 0.72042316457491791906e-3_f64 * t35256 + 0.12769379967989351819e-4_f64 * t45742 - 0.25538759935978703638e-4_f64 * t45744 - 0.25538759935978703638e-4_f64 * t45746 + 0.23942587439980034662e-4_f64 * t45750 + 0.1064114997332445985e-4_f64 * t45752 - 0.3192344991997337955e-4_f64 * t45754;
    t45756
}
