//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 878/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk878(t10339: f64, t10424: f64, t10426: f64, t1102: f64, t14232: f64, t14235: f64, t14250: f64, t14260: f64, t14272: f64, t14299: f64, t14339: f64, t14341: f64, t14343: f64, t18775: f64, t18780: f64, t18784: f64, t18789: f64, t18794: f64, t18800: f64, t18859: f64, t19107: f64, t278: f64, t344: f64) -> f64 {
    let t19110 = 0.492782225e-3_f64 * t1102 * t18775 - 0.1478346675e-2_f64 * t1102 * t18780 + 0.59133867e-2_f64 * t1102 * t18784 + t14232 + t14235 + 0.7391733375e-3_f64 * t1102 * t18789 - 0.1478346675e-2_f64 * t1102 * t18794 + 0.17521145777777777778e-2_f64 * t14250 - 0.14600954814814814815e-3_f64 * t10339 - t14260 - 0.65704296666666666667e-3_f64 * t14272 + 0.492782225e-3_f64 * t18800 + 0.43802864444444444443e-3_f64 * t14299 - 0.98556445e-3_f64 * t344 * t18859 - 0.32852148333333333333e-3_f64 * t10424 + 0.21901432222222222222e-3_f64 * t10426 - t14339 - t14341 + t14343 - 4.0_f64 * t278 * t19107;
    t19110
}
