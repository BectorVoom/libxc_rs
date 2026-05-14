//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 804/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk804<F: Float>(t8117: F, t8174: F, t8220: F, t8256: F, t8303: F, t8343: F, t8392: F, t8432: F, t2967: F, t747: F, t1052: F, t1961: F, t7346: F, t7349: F, t7352: F, t7356: F, t7360: F, t7364: F, t7373: F, t7376: F, t7380: F, t7385: F, t7388: F, t7392: F, t7398: F, t7414: F, t7417: F, t7421: F, t7424: F, t7430: F) -> (F, F, F, F) {
    let t8435 = t8117 + t8174 + t8220 + t8256 + t8303 + t8343 + t8392 + t8432;
    let t8440 = t2967 * t747;
    let t8443 = t1052 * t1961;
    let t8466 = 0.19171462976960374838e0 * t7346 - 0.51123901271894332902e0 * t7349 + 0.1022478025437886658e1 * t7352 - 0.1022478025437886658e1 * t7356 + 0.17041300423964777634e0 * t7360 - 0.17041300423964777634e0 * t7364 - 0.59584149919750711116e-1 * t7373 + 0.38342925953920749676e0 * t7376 - 0.85206502119823888168e-1 * t7380 - 0.38342925953920749676e0 * t7385 + 0.38342925953920749676e0 * t7388 + 0.59584149919750711116e-1 * t7392 - 0.38342925953920749676e0 * t7398 - 0.76685851907841499352e0 * t7414 + 0.76685851907841499352e0 * t7417 - 0.76685851907841499352e0 * t7421 + 0.19171462976960374838e1 * t7424 - 0.11502877786176224903e1 * t7430;
    (t8435, t8440, t8443, t8466)
}
