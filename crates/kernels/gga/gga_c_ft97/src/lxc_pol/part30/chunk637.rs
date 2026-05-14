//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 637/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk637<F: Float>(t191: F, t295: F, t309: F, t10696: F, t1501: F, t4181: F, t15195: F, t6361: F, t4162: F, t6360: F, t15369: F, t25271: F, t4167: F, t15460: F, t1901: F, t24903: F, t29098: F, t29101: F, t29104: F, t29107: F, t29111: F, t29113: F, t29116: F, t29120: F, t29124: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t29127 = t191 * t295;
    let t29128 = t29127 * t309;
    let t29129 = t10696 * t1501;
    let t29130 = t29129 * t4181;
    let t29131 = t29128 * t29130;
    let t29134 = t15195 * t6361;
    let t29137 = t6360 * t4162;
    let t29138 = t15369 * t29137;
    let t29141 = t25271 * t4167;
    let t29142 = t15460 * t29141;
    let t29145 = t1901 * t29098 / 9.0 - t446 * t29101 / 3.0 - t446 * t29104 / 3.0 + t1901 * t29107 / 9.0 - t24903 / 27.0 - t29111 / 27.0 + t1901 * t29113 / 9.0 - t446 * t29116 / 3.0 - t446 * t29120 / 3.0 - 2.0 / 3.0 * t1901 * t29124 - 2.0 * t1901 * t29131 + t1901 * t29134 / 9.0 - 2.0 / 3.0 * t1901 * t29138 - 2.0 / 3.0 * t1901 * t29142;
    (t29127, t29128, t29129, t29130, t29137, t29141, t29145)
}
