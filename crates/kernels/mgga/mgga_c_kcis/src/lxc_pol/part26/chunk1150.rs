//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1150/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1150<F: Float>(t1464: F, t28360: F, t98409: F, t28356: F, t28382: F, t2038: F, t28503: F, t5627: F, t1615: F, t27596: F, t6176: F, t7497: F, t1394: F, t7100: F, t94216: F, t22285: F, t27387: F) -> (F, F, F, F, F, F) {
    let t101994 = t1464 * t98409 * t28360;
    let t101997 = t1464 * t28356 * t28382;
    let t102001 = t1464 * t28503 * t2038 * t5627;
    let t102005 = t6176 * t27596 * t7497 * t1615;
    let t102011 = t1394 * t94216 * t7100;
    let t102014 = t1394 * t27387 * t22285;
    (t101994, t101997, t102001, t102005, t102011, t102014)
}
