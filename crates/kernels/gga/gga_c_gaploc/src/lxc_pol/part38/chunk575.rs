//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 575/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk575<F: Float>(t11177: F, t549: F, t1: F, t11279: F, t544: F, t11186: F, t11181: F, t11173: F, t1457: F, t11241: F, t11168: F, t10400: F, t10403: F, t10410: F, t1424: F, t1456: F, t1572: F, t4386: F, t4391: F, t4507: F, t9422: F, t9442: F) -> (F, F, F, F, F, F, F) {
    let t11422 = t549 * t11177;
    let t11425 = t11279 * t1;
    let t11426 = t544 * t11425;
    let t11429 = t11186 * t1;
    let t11430 = t544 * t11429;
    let t11433 = t11181 * t1;
    let t11434 = t544 * t11433;
    let t11440 = t1457 * t11173;
    let t11443 = t1457 * t11241;
    let t11446 = t1457 * t11168;
    let t11451 = -0.79445533226334281487e-1 * t4391 * t11422 + 0.11916829983950142223e0 * t11426 * t4386 - 0.39722766613167140743e-1 * t11430 * t1424 - 0.39722766613167140743e-1 * t11434 * t1424 + 0.76685851907841499353e0 * t10400 - 0.76685851907841499353e0 * t10403 - 0.15337170381568299871e1 * t10410 - 0.71500979903700853338e0 * t4507 * t11440 + 0.14300195980740170668e1 * t1572 * t11443 + 0.35750489951850426669e0 * t1456 * t11446 + 0.12780975317973583225e0 * t9422 + 0.31952438294933958063e-1 * t9442;
    (t11425, t11426, t11429, t11430, t11433, t11434, t11451)
}
