//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1229/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1229<F: Float>(t8246: F, t8254: F, t8243: F, t8251: F, t2723: F, t7244: F, t10935: F, t24978: F, t25335: F, t25338: F, t25343: F, t25346: F, t25348: F, t25353: F, t25357: F, t25361: F, t25365: F, t25369: F, t2725: F, t2812: F, t2813: F, t2814: F, t297: F, t312: F, t8198: F, t8217: F, t894: F, t940: F) -> (F, F) {
    let t25377 = t8254 * t8246;
    let t25379 = t8251 * t8243;
    let t25381 = t2723 * t7244;
    let t25385 = F::cast_from(0.36282051390366161644e7_f64) * t25335 * t8217 - F::cast_from(0.60470085650610269407e6_f64) * t25338 * t8198 + F::cast_from(0.35163949364965747848e4_f64) * t25343 - F::cast_from(0.17581974682482873924e4_f64) * t25346 + F::cast_from(0.69310201356862480534e2_f64) * t2812 * t10935 * t25348 + F::cast_from(0.519826510176468604e2_f64) * t25353 - F::cast_from(0.41296608323992124631e2_f64) * t25357 + F::cast_from(0.44430618325890501511e2_f64) * t25361 * t2725 + F::cast_from(0.1559479530529405812e2_f64) * t2812 * t2813 * t25365 + F::cast_from(0.15802725909364645561e4_f64) * t25369 * t2814 + F::cast_from(0.5848048239485271795e1_f64) * t940 * t894 * t312 * t24978 * t297 - F::cast_from(0.99111859977581099115e3_f64) * t25377 + F::cast_from(0.33037286659193699704e3_f64) * t25379 + F::cast_from(0.1559479530529405812e2_f64) * t2812 * t2813 * t25381;
    (t25381, t25385)
}
