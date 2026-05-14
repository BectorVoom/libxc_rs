//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 836/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk836<F: Float>(t35242: F, t35246: F, t35256: F, t45716: F, t45722: F, t45724: F, t45728: F, t45732: F, t45734: F, t45736: F, t45738: F, t45742: F, t45744: F, t45746: F, t45750: F, t45752: F, t45754: F) -> (F,) {
    let t45756 = -0.5124043883133942371e-4 * t45716 + 0.30487649791575028314e-3 * t35242 - 0.43368970657079495312e-4 * t35246 + 0.40911992481368012592e-1 * t45722 - 0.81823984962736025184e-1 * t45724 - 0.81823984962736025184e-1 * t45728 - 0.81823984962736025184e-1 * t45732 + 0.25538759935978703638e-4 * t45734 + 0.25538759935978703638e-4 * t45736 + 0.12769379967989351819e-4 * t45738 - 0.72042316457491791906e-3 * t35256 + 0.12769379967989351819e-4 * t45742 - 0.25538759935978703638e-4 * t45744 - 0.25538759935978703638e-4 * t45746 + 0.23942587439980034662e-4 * t45750 + 0.1064114997332445985e-4 * t45752 - 0.3192344991997337955e-4 * t45754;
    (t45756,)
}
