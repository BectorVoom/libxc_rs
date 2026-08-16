//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 636/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk636(t11177: f64, t549: f64, t1: f64, t11279: f64, t544: f64, t11186: f64, t11181: f64, t11173: f64, t1457: f64, t11241: f64, t11168: f64, t10400: f64, t10403: f64, t10410: f64, t1424: f64, t1456: f64, t1572: f64, t4386: f64, t4391: f64, t4507: f64, t9422: f64, t9442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
    let t11451 = -0.79445533226334281487e-1_f64 * t4391 * t11422 + 0.11916829983950142223e0_f64 * t11426 * t4386 - 0.39722766613167140743e-1_f64 * t11430 * t1424 - 0.39722766613167140743e-1_f64 * t11434 * t1424 + 0.76685851907841499353e0_f64 * t10400 - 0.76685851907841499353e0_f64 * t10403 - 0.15337170381568299871e1_f64 * t10410 - 0.71500979903700853338e0_f64 * t4507 * t11440 + 0.14300195980740170668e1_f64 * t1572 * t11443 + 0.35750489951850426669e0_f64 * t1456 * t11446 + 0.12780975317973583225e0_f64 * t9422 + 0.31952438294933958063e-1_f64 * t9442;
    (t11425, t11426, t11429, t11430, t11433, t11434, t11451)
}
