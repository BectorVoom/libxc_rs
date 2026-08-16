//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1110/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1110(t2617: f64, t3726: f64, t7803: f64, t43433: f64, t43435: f64, t43440: f64, t43442: f64, t47180: f64, t47186: f64, t47191: f64, t47193: f64, t47196: f64, t47199: f64, t47203: f64) -> f64 {
    let t47206 = t7803 * t3726 * t2617;
    let t47208 = -t43433 - 0.38342925953920749676e0_f64 * t43435 - 0.44688112439813033337e-1_f64 * t47180 - t47186 + t47191 + t43440 - 0.19171462976960374838e0_f64 * t43442 + 0.11916829983950142223e0_f64 * t47193 + 0.29792074959875355558e-1_f64 * t47196 - 0.14896037479937677779e-1_f64 * t47199 - 0.39722766613167140743e-1_f64 * t47203 + 0.19171462976960374838e0_f64 * t47206;
    t47208
}
