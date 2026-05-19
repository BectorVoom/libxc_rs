//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1202/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1202<F: Float>(t2724: F, t4364: F, t4365: F, t2482: F, t2719: F, t814: F, t14671: F, t14686: F, t4366: F, t10891: F, t10893: F, t10906: F, t14894: F, t14896: F, t14900: F, t14904: F, t14907: F, t14910: F, t14914: F, t14919: F, t14925: F, t2745: F, t4362: F) -> F {
    let t14927 = t4364 * t4365 * t2724;
    let t14931 = t2482 * t2719 * t814;
    let t14933 = t14686 * t14671 * t4366;
    let t14934 = t14931 * t14933;
    let t14936 = -F::new(35.0) / F::new(108.0) * t10891 + F::new(7.0) / F::new(144.0) * t10893 - F::new(7.0) / F::new(48.0) * t10906 - F::cast_from(0.12862205435420921092e-2_f64) * t14894 * t14896 - F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t14900 + F::cast_from(0.85748036236139473944e-3_f64) * t4362 * t14904 - F::cast_from(0.80031500487063509014e-2_f64) * t14907 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t14910 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t14914 - F::cast_from(0.42874018118069736972e-2_f64) * t2745 * t14919 - t14925 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t14927 + F::cast_from(0.50820002809285328225e-4_f64) * t14934;
    t14936
}
