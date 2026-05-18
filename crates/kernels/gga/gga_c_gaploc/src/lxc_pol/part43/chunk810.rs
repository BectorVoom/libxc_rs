//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 810/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk810<F: Float>(t28013: F, t5641: F, t883: F, t9805: F, t23000: F, t27997: F, t2624: F, t9800: F, t9829: F, t2617: F, t3255: F, t7803: F) -> (F, F, F, F) {
    let t40956 = t9805 * t5641 * t883 * t28013;
    let t40966 = t23000 * t5641 * t883 * t27997;
    let t40969 = t9800 * t2624 * t9829;
    let t40986 = t7803 * t3255 * t2617;
    (t40956, t40966, t40969, t40986)
}
