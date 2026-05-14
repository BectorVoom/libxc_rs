//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 495/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk495<F: Float>(t3773: F, t4162: F, t504: F, t1455: F, t1458: F, t1520: F, t1457: F, t503: F, t475: F, t3502: F, t382: F, t487: F, t486: F, t1297: F, t391: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4163 = t3773 + t4162;
    let t4164 = t4163 * t504;
    let t4165 = t1455 * t1458;
    let t4167 = 2.0 * t4165 * t1520;
    let t4169 = 1.0 / t1457 / t503;
    let t4170 = t475 * t4169;
    let t4171 = t1520 * t1520;
    let t4173 = 2.0 * t4170 * t4171;
    let t4174 = t382 * t3502;
    let t4175 = t487 * t4174;
    let t4176 = t486 * t4175;
    let t4180 = 1.0 / t391 / t494 / t1297;
    (t4163, t4164, t4165, t4167, t4169, t4170, t4171, t4173, t4174, t4175, t4176, t4180)
}
