//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1045/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1045<F: Float>(t10359: F, t552: F, t551: F, t549: F, t6415: F, t6424: F, t8147: F, t8149: F, t8151: F, t8154: F, t8201: F, t8227: F, t9391: F, t9397: F, t9401: F, t9416: F, t9420: F, t9424: F, t9431: F, t9436: F) -> (F, F) {
    let t10360 = t552 * t10359;
    let t10361 = t551 * t10360;
    let t10364 = -0.69345773920434148506e0 * t9391 + 0.19043987679069580388e-1 * t8147 + 0.48787202696913915093e-3 * t8149 + 0.87816964854445047168e-1 * t8151 + 0.2037639021386884617e0 * t8154 + 0.82318114786693894983e-2 * t9397 + 0.34930954652346593433e-1 * t9401 + t6415 + t6424 - 0.1047928639570397803e0 * t9416 - 0.52396431978519890151e-1 * t9420 + 0.82318114786693894983e-2 * t9424 - 0.34930954652346593433e-1 * t9431 - 0.17465477326173296717e-1 * t9436 + 0.12713391885412927226e1 * t8201 - 0.4939086887201633699e-1 * t8227 - 0.43341108700271342816e-1 * t549 * t10361;
    (t10361, t10364)
}
