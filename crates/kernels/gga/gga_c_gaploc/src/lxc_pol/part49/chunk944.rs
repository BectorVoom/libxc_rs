//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 944/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk944<F: Float>(t3177: F, t35091: F, t9272: F, t204: F, t41726: F, t587: F, t2487: F, t6711: F, t10532: F, t10533: F, t41749: F, t41810: F, t6716: F, t6717: F) -> (F, F, F, F, F) {
    let t42226 = t9272 * t35091 * t3177;
    let t42227 = F::new(0.11502877786176224903e1) * t42226;
    let t42230 = F::new(0.18404604457881959845e2) * t587 * t204 * t41726;
    let t42233 = F::new(0.14953741122029092374e3) * t2487 * t6711 * t41726;
    let t42236 = F::new(0.55213813373645879534e2) * t10532 * t10533 * t41749;
    let t42239 = F::new(0.69017266717057349418e1) * t6716 * t6717 * t41810;
    (t42227, t42230, t42233, t42236, t42239)
}
