//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1051/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1051<F: Float>(t27059: F, t448: F, t429: F, t745: F, t116: F, t428: F, t2849: F, t371: F, t26336: F, t3086: F, t8428: F, t1113: F, t8414: F) -> (F, F, F, F, F, F, F) {
    let t27061 = F::cast_from(0.18781521737197933637e-2_f64) * t448 * t27059;
    let t27071 = t745 * t429;
    let t27074 = F::new(5.0) / F::new(486.0) * t428 * t116 * t27071;
    let t27082 = F::new(1.0) / t371 / t2849;
    let t27083 = t27082 * t26336;
    let t27100 = t3086 * t8428;
    let t27112 = t1113 * t8414;
    (t27061, t27071, t27074, t27082, t27083, t27100, t27112)
}
