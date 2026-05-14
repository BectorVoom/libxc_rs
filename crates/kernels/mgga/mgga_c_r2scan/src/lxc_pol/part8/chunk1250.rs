//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1250/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1250<F: Float>(t1524: F, t3142: F, t1416: F, t3037: F, t1385: F, t2: F, t464: F, t8590: F, t1411: F, t406: F, t8635: F, t410: F, t1419: F, t8637: F, t1531: F, t8553: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28048 = t3142 * t1524;
    let t28051 = t1416 * t3037;
    let t28063 = t3142 * t1385;
    let t28069 = t8590 * t2 * t464;
    let t28086 = t3142 * t1411;
    let t28088 = t406 * t8635;
    let t28090 = t410 * t8635;
    let t28095 = t1419 * t3037;
    let t28102 = t410 * t8637;
    let t28104 = t8553 * t1531;
    (t28048, t28051, t28063, t28069, t28086, t28088, t28090, t28095, t28102, t28104)
}
