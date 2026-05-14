//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 994/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk994<F: Float>(t2375: F, t47838: F, t41650: F, t41654: F, t41657: F, t41661: F, t41664: F, t41667: F, t47823: F, t47827: F, t47829: F, t47832: F, t47835: F, t2386: F, t3689: F, t544: F, t6514: F) -> (F, F) {
    let t47839 = t47838 * t2375;
    let t47842 = t47823 - t47827 + 0.19171462976960374838e0 * t47829 - 0.38342925953920749676e0 * t47832 - 0.79445533226334281487e-1 * t47835 + 0.11916829983950142223e0 * t47839 + t41650 + t41654 - t41657 + t41661 - 0.39722766613167140743e-1 * t41664 - t41667;
    let t47846 = t544 * t6514 * t3689 * t2386;
    (t47842, t47846)
}
