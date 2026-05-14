//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1383/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1383<F: Float>(t22426: F, t22431: F, t22433: F, t22437: F, t26790: F, t28833: F, t28836: F, t28838: F, t28840: F, t28843: F, t28846: F, t28848: F, t28850: F, t7654: F, t8987: F, t21958: F, t21963: F, t22441: F, t22443: F, t22446: F, t22449: F, t22450: F, t22454: F, t22459: F, t26813: F, t26814: F, t26820: F, t26824: F) -> (F, F) {
    let t33677 = -0.14447919941302971324e1 * t26790 + 0.17315859105681463759e2 * t22426 + 0.51947577317044391277e2 * t22431 + 0.30762056574649219973e4 * t22433 + 0.6233709278045326953e3 * t22437 + 0.30762056574649219973e4 * t28833 + 0.4051561992e0 * t28836 - 0.35089341735807877242e1 * t28838 + 0.51947577317044391277e2 * t28840 + 0.4051561992e0 * t28843 + 0.5143752e0 * t28846 - 0.70178683471615754484e1 * t28848 + 0.10389515463408878255e3 * t28850;
    let t33681 = t8987 * t7654;
    let t33686 = 120.0 * t22441 + 0.65061487801810439052e-1 * t22443 + t22446 - t22449 - 0.62254000682014814811e-2 * t22450 + 0.4051561992e0 * t33681 + t26813 + 0.19518446340543131715e0 * t26814 - 0.11558335953042377058e2 * t22454 - t21958 - t21963 - 0.16867793133802706421e-1 * t22459 - t26820 + t26824;
    (t33677, t33686)
}
