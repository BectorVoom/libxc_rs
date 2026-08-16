//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1017/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1017(t2365: f64, t33087: f64, t8775: f64, t41012: f64, t41015: f64, t41019: f64, t10639: f64, t10912: f64, t787: f64, t899: f64, t913: f64, t11001: f64, t9823: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43446 = t8775 * t2365 * t33087;
    let t43447 = 0.20854452471912748891e0_f64 * t43446;
    let t43448 = 0.19171462976960374838e1_f64 * t41012;
    let t43449 = 0.11502877786176224903e1_f64 * t41015;
    let t43450 = 0.46011511144704899612e1_f64 * t41019;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43455 = 0.17875244975925213335e0_f64 * t43454;
    let t43456 = t9823 * t11001;
    (t43447, t43448, t43449, t43450, t43455, t43456)
}
