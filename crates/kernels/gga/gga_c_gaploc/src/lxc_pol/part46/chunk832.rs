//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 832/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk832<F: Float>(t43432: F, t2679: F, t3451: F, t9796: F, t3038: F, t6119: F, t787: F, t9755: F, t11112: F, t2617: F, t7810: F, t41008: F, t2365: F, t33087: F, t8775: F, t41012: F) -> (F, F, F, F, F, F, F) {
    let t43433 = 0.59584149919750711116e-1 * t43432;
    let t43435 = t9796 * t3451 * t2679;
    let t43440 = 0.27805936629216998521e0 * t787 * t9755 * t3038 * t6119;
    let t43442 = t7810 * t11112 * t2617;
    let t43444 = 0.10352590007558602413e2 * t41008;
    let t43446 = t8775 * t2365 * t33087;
    let t43447 = 0.20854452471912748891e0 * t43446;
    let t43448 = 0.19171462976960374838e1 * t41012;
    (t43433, t43435, t43440, t43442, t43444, t43447, t43448)
}
