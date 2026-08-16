//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 602/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk602<F: Float>(t11177: F, t549: F, t1: F, t11279: F, t544: F, t11186: F, t11181: F, t11173: F, t1457: F, t11241: F, t11168: F, t10400: F, t10403: F, t10410: F, t1424: F, t1456: F, t1572: F, t4386: F, t4391: F, t4507: F, t9422: F, t9442: F) -> (F, F, F, F, F, F, F) {
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
    let t11451 = -F::cast_from(0.79445533226334281487e-1_f64) * t4391 * t11422 + F::cast_from(0.11916829983950142223e0_f64) * t11426 * t4386 - F::cast_from(0.39722766613167140743e-1_f64) * t11430 * t1424 - F::cast_from(0.39722766613167140743e-1_f64) * t11434 * t1424 + F::cast_from(0.76685851907841499353e0_f64) * t10400 - F::cast_from(0.76685851907841499353e0_f64) * t10403 - F::cast_from(0.15337170381568299871e1_f64) * t10410 - F::cast_from(0.71500979903700853338e0_f64) * t4507 * t11440 + F::cast_from(0.14300195980740170668e1_f64) * t1572 * t11443 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t11446 + F::cast_from(0.12780975317973583225e0_f64) * t9422 + F::cast_from(0.31952438294933958063e-1_f64) * t9442;
    (t11425, t11426, t11429, t11430, t11433, t11434, t11451)
}
