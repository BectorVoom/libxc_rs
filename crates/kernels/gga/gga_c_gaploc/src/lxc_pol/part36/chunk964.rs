//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 964/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk964<F: Float>(t41008: F, t2365: F, t33087: F, t8775: F, t41012: F, t41015: F, t41019: F, t10639: F, t10912: F, t787: F, t899: F, t913: F) -> (F, F, F, F, F, F) {
    let t43444 = F::new(0.10352590007558602413e2) * t41008;
    let t43446 = t8775 * t2365 * t33087;
    let t43447 = F::new(0.20854452471912748891e0) * t43446;
    let t43448 = F::new(0.19171462976960374838e1) * t41012;
    let t43449 = F::new(0.11502877786176224903e1) * t41015;
    let t43450 = F::new(0.46011511144704899612e1) * t41019;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    (t43444, t43447, t43448, t43449, t43450, t43454)
}
