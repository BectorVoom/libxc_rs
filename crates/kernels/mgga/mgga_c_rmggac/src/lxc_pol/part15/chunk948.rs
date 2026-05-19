//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 948/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk948<F: Float>(t2289: F, t9087: F, t2412: F, t8592: F, t2410: F, t3350: F, t8515: F, t8519: F, t39277: F, t8668: F, t8831: F, t35242: F, t35246: F, t35256: F, t45716: F, t45722: F, t45724: F, t45728: F, t45732: F, t45734: F, t45736: F, t45738: F, t45742: F) -> F {
    let t45744 = t9087 * t2289;
    let t45746 = t2412 * t8592;
    let t45750 = t2410 * t8515 * t3350 * t8519;
    let t45752 = t39277 * t8668;
    let t45754 = t39277 * t8831;
    let t45756 = -F::cast_from(0.5124043883133942371e-4_f64) * t45716 + F::cast_from(0.30487649791575028314e-3_f64) * t35242 - F::cast_from(0.43368970657079495312e-4_f64) * t35246 + F::cast_from(0.40911992481368012592e-1_f64) * t45722 - F::cast_from(0.81823984962736025184e-1_f64) * t45724 - F::cast_from(0.81823984962736025184e-1_f64) * t45728 - F::cast_from(0.81823984962736025184e-1_f64) * t45732 + F::cast_from(0.25538759935978703638e-4_f64) * t45734 + F::cast_from(0.25538759935978703638e-4_f64) * t45736 + F::cast_from(0.12769379967989351819e-4_f64) * t45738 - F::cast_from(0.72042316457491791906e-3_f64) * t35256 + F::cast_from(0.12769379967989351819e-4_f64) * t45742 - F::cast_from(0.25538759935978703638e-4_f64) * t45744 - F::cast_from(0.25538759935978703638e-4_f64) * t45746 + F::cast_from(0.23942587439980034662e-4_f64) * t45750 + F::cast_from(0.1064114997332445985e-4_f64) * t45752 - F::cast_from(0.3192344991997337955e-4_f64) * t45754;
    t45756
}
