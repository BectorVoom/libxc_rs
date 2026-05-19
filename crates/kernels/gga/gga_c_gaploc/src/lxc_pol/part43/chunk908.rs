//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 908/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk908<F: Float>(t43446: F, t41012: F, t41015: F, t41019: F, t10639: F, t10912: F, t787: F, t899: F, t913: F, t13118: F, t15362: F, t2365: F, t32357: F, t6111: F) -> (F, F, F, F, F, F, F) {
    let t43447 = F::cast_from(0.20854452471912748891e0_f64) * t43446;
    let t43448 = F::cast_from(0.19171462976960374838e1_f64) * t41012;
    let t43449 = F::cast_from(0.11502877786176224903e1_f64) * t41015;
    let t43450 = F::cast_from(0.46011511144704899612e1_f64) * t41019;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43455 = F::cast_from(0.17875244975925213335e0_f64) * t43454;
    let t43464 = t15362 * t13118;
    let t43465 = F::cast_from(0.59584149919750711116e-1_f64) * t43464;
    let t43467 = t6111 * t2365 * t32357;
    (t43447, t43448, t43449, t43450, t43455, t43465, t43467)
}
