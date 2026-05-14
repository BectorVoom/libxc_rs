//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1071/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1071<F: Float>(t39485: F, t39499: F, t39502: F, t37616: F, t37619: F, t37630: F, t37634: F, t37639: F, t37656: F, t39482: F, t39490: F, t39495: F, t39511: F, t39522: F, t39506: F, t39509: F, t39514: F, t39517: F, t39520: F, t39524: F, t39526: F, t39529: F, t39532: F, t39535: F) -> (F, F) {
    let t41405 = 0.93443229163669953711e-1 * t39485;
    let t41414 = 0.46230515946956099004e0 * t39499;
    let t41415 = 0.1536604809351619373e1 * t39502;
    let t41416 = 0.31147743054556651237e-1 * t39482 + t41405 - 0.16951189180550569635e1 * t37616 + 0.23115257973478049502e0 * t37619 + 0.17336443480108537126e0 * t39490 - 0.23804984598836975486e0 * t37630 - 0.71414953796510926458e0 * t37634 - 0.57829097596741960692e-3 * t37639 + 0.21951497276451705328e0 * t39495 - 0.97574405393827830187e-2 * t37656 + t41414 - t41415;
    let t41419 = 0.25610080155860322884e0 * t39511;
    let t41423 = 0.46230515946956099004e0 * t39522;
    let t41429 = 0.65854491829355115984e0 * t39506 + 0.32927245914677557992e0 * t39509 + t41419 - 0.54878743191129263322e-1 * t39514 + 0.43663693315433241794e-2 * t39517 + 0.52396431978519890152e-1 * t39520 + t41423 + 0.10401866088065122276e1 * t39524 - 0.17465477326173296718e-1 * t39526 - 0.17465477326173296718e-1 * t39529 + 0.26198215989259945076e-1 * t39532 - 0.26198215989259945077e-1 * t39535;
    (t41416, t41429)
}
