//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1194/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1194<F: Float>(t24378: F, t25070: F, t28599: F, t111837: F, t2691: F, t108834: F, t108969: F, t109221: F, t112073: F, t112202: F, t112212: F, t112226: F, t112237: F, t14742: F, t14766: F, t2409: F, t28552: F, t28598: F, t28603: F, t54928: F, t6035: F, t70779: F, t98570: F) -> (F,) {
    let t112266 = t25070 * t24378 * t28599;
    let t112268 = t2691 * t111837;
    let t112279 = 0.13335600218518518519e0 * t28552 * t108834 - 0.13335600218518518519e0 * t28552 * t109221 - 0.33339000546296296298e-1 * t98570 - 0.80559205902449556554e-1 * t28603 * t108969 + 0.66678001092592592594e-1 * t25070 * t6035 * t28598 * t2409 - 0.22226000364197530864e-1 * t112266 + 0.96671047082939467864e0 * t112268 * t112073 + 0.90613700826057446696e0 * t14742 * t112237 - 0.48327307107230638236e1 * t14766 * t112202 - 0.13592055123908617004e1 * t54928 * t112212 - 0.43791161479435967988e1 * t70779 * t112226;
    (t112279,)
}
