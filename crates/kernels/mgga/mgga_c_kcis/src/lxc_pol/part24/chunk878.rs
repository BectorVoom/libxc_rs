//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 878/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk878<F: Float>(t10339: F, t10424: F, t10426: F, t1102: F, t14232: F, t14235: F, t14250: F, t14260: F, t14272: F, t14299: F, t14339: F, t14341: F, t14343: F, t18775: F, t18780: F, t18784: F, t18789: F, t18794: F, t18800: F, t18859: F, t19107: F, t278: F, t344: F) -> F {
    let t19110 = F::cast_from(0.492782225e-3_f64) * t1102 * t18775 - F::cast_from(0.1478346675e-2_f64) * t1102 * t18780 + F::new(0.59133867e-2) * t1102 * t18784 + t14232 + t14235 + F::cast_from(0.7391733375e-3_f64) * t1102 * t18789 - F::cast_from(0.1478346675e-2_f64) * t1102 * t18794 + F::cast_from(0.17521145777777777778e-2_f64) * t14250 - F::cast_from(0.14600954814814814815e-3_f64) * t10339 - t14260 - F::cast_from(0.65704296666666666667e-3_f64) * t14272 + F::cast_from(0.492782225e-3_f64) * t18800 + F::cast_from(0.43802864444444444443e-3_f64) * t14299 - F::new(0.98556445e-3) * t344 * t18859 - F::cast_from(0.32852148333333333333e-3_f64) * t10424 + F::cast_from(0.21901432222222222222e-3_f64) * t10426 - t14339 - t14341 + t14343 - F::new(4.0) * t278 * t19107;
    t19110
}
