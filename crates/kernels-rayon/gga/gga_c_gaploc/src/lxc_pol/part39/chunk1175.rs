//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1175/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1175(t107: f64, t47008: f64, t544: f64, t2375: f64, t41650: f64, t41654: f64, t41657: f64, t41661: f64, t41664: f64, t41667: f64, t47823: f64, t47827: f64, t47829: f64, t47832: f64, t47835: f64) -> f64 {
    let t47838 = t544 * t47008 * t107;
    let t47839 = t47838 * t2375;
    let t47842 = t47823 - t47827 + 0.19171462976960374838e0_f64 * t47829 - 0.38342925953920749676e0_f64 * t47832 - 0.79445533226334281487e-1_f64 * t47835 + 0.11916829983950142223e0_f64 * t47839 + t41650 + t41654 - t41657 + t41661 - 0.39722766613167140743e-1_f64 * t41664 - t41667;
    t47842
}
