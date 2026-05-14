//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1117/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1117<F: Float>(t2723: F, t7244: F, t10935: F, t24978: F, t25335: F, t25338: F, t25343: F, t25346: F, t25348: F, t25353: F, t25357: F, t25361: F, t25365: F, t25369: F, t25377: F, t25379: F, t2725: F, t2812: F, t2813: F, t2814: F, t297: F, t312: F, t8198: F, t8217: F, t894: F, t940: F) -> (F, F) {
    let t25381 = t2723 * t7244;
    let t25385 = 0.36282051390366161644e7 * t25335 * t8217 - 0.60470085650610269407e6 * t25338 * t8198 + 0.35163949364965747848e4 * t25343 - 0.17581974682482873924e4 * t25346 + 0.69310201356862480534e2 * t2812 * t10935 * t25348 + 0.519826510176468604e2 * t25353 - 0.41296608323992124631e2 * t25357 + 0.44430618325890501511e2 * t25361 * t2725 + 0.1559479530529405812e2 * t2812 * t2813 * t25365 + 0.15802725909364645561e4 * t25369 * t2814 + 0.5848048239485271795e1 * t940 * t894 * t312 * t24978 * t297 - 0.99111859977581099115e3 * t25377 + 0.33037286659193699704e3 * t25379 + 0.1559479530529405812e2 * t2812 * t2813 * t25381;
    (t25381, t25385)
}
