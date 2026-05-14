//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 929/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk929<F: Float>(t1349: F, t1362: F, t1389: F, t2081: F, t23474: F, t23479: F, t23501: F, t23537: F, t23939: F, t23964: F, t23989: F, t23993: F, t23998: F, t24104: F, t24118: F, t24119: F, t24122: F, t24127: F, t24131: F, t24135: F, t24139: F, t24143: F, t24148: F, t5772: F) -> (F,) {
    let t24151 = -t2081 * t1389 - t5772 * t24104 / 9.0 - 2.0 * t23993 - 2.0 * t23474 - 4.0 * t23479 - 2.0 * t23939 - 4.0 * t23989 + 4.0 * t23998 + 8.0 * t23964 - 12.0 * t23501 + 4.0 * t23537 + t24118 - t24119 / 9.0 + t1349 * t24122 / 3.0 + t1349 * t24127 / 6.0 + t1349 * t24131 / 6.0 - t5772 * t24135 / 18.0 - t5772 * t24139 / 27.0 + t5772 * t24143 / 9.0 + t24148 * t1362 / 6.0;
    (t24151,)
}
