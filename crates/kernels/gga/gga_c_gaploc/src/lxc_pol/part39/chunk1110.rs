//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1110/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1110<F: Float>(t2617: F, t3726: F, t7803: F, t43433: F, t43435: F, t43440: F, t43442: F, t47180: F, t47186: F, t47191: F, t47193: F, t47196: F, t47199: F, t47203: F) -> F {
    let t47206 = t7803 * t3726 * t2617;
    let t47208 = -t43433 - F::new(0.38342925953920749676e0) * t43435 - F::new(0.44688112439813033337e-1) * t47180 - t47186 + t47191 + t43440 - F::new(0.19171462976960374838e0) * t43442 + F::new(0.11916829983950142223e0) * t47193 + F::new(0.29792074959875355558e-1) * t47196 - F::new(0.14896037479937677779e-1) * t47199 - F::new(0.39722766613167140743e-1) * t47203 + F::new(0.19171462976960374838e0) * t47206;
    t47208
}
