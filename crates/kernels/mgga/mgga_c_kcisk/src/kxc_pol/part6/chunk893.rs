//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 893/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk893<F: Float>(t1421: F, t16885: F, t16897: F, t22646: F, t22652: F, t22654: F, t22656: F, t28886: F, t28894: F, t28898: F, t28902: F, t28906: F, t456: F) -> F {
    let t28909 = -F::new(0.98556445e-3) * t16885 - F::cast_from(0.43802864444444444445e-3_f64) * t16897 - F::cast_from(0.36958666875e-3_f64) * t456 * t28886 + F::cast_from(0.21901432222222222222e-2_f64) * t22646 - F::cast_from(0.26281718666666666667e-2_f64) * t22652 + F::cast_from(0.13140859333333333334e-2_f64) * t22654 - F::new(0.59133867e-2) * t22656 + F::cast_from(0.29201909629629629629e-2_f64) * t1421 * t28894 + F::new(0.59133867e-2) * t1421 * t28898 + F::cast_from(0.39422577999999999999e-2_f64) * t1421 * t28902 - F::cast_from(0.65704296666666666666e-2_f64) * t1421 * t28906;
    t28909
}
