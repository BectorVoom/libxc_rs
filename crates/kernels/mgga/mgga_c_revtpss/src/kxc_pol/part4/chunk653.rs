//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 653/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk653<F: Float>(t3181: F, t66: F, t2853: F, t247: F, t1041: F, t1047: F, t1063: F, t1068: F, t3082: F, t3083: F, t3086: F, t3091: F, t3097: F, t3101: F, t3106: F, t3112: F, t3115: F, t3120: F, t3124: F, t3127: F, t3130: F, t3136: F, t3150: F, t3157: F, t3161: F, t3164: F, t3169: F, t3174: F, t3177: F, t348: F) -> (F, F, F) {
    let t3182 = t66 * t3181;
    let t3183 = t3182 * t2853;
    let t3184 = t247 * t3183;
    let t3187 = -t3082 + F::new(11.0) / F::new(108.0) * t3083 * t348 - t3086 / F::new(54.0) + F::new(0.28582678745379824648e-3) * t3091 * t3097 - F::new(0.28582678745379824648e-3) * t1063 * t3101 - F::new(0.15244095330869239812e-2) * t3106 * t1068 + F::new(0.19055119163586549765e-3) * t3112 - F::new(0.42874018118069736972e-3) * t3115 * t3120 + F::new(0.42874018118069736972e-3) * t3124 * t1047 - F::new(0.28582678745379824648e-3) * t3127 * t3130 + F::new(0.21437009059034868486e-3) * t1041 * t3136 + F::new(0.42874018118069736972e-3) * t3150 * t3157 - F::new(0.21437009059034868486e-3) * t3161 * t3164 - F::new(0.22866142996303859718e-2) * t3169 * t1047 + F::new(0.28582678745379824648e-3) * t3174 + F::new(0.14291339372689912324e-3) * t1063 * t3177 + F::new(0.23818898954483187207e-3) * t1063 * t3184;
    (t3182, t3184, t3187)
}
