//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1229/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1229(t8246: f64, t8254: f64, t8243: f64, t8251: f64, t2723: f64, t7244: f64, t10935: f64, t24978: f64, t25335: f64, t25338: f64, t25343: f64, t25346: f64, t25348: f64, t25353: f64, t25357: f64, t25361: f64, t25365: f64, t25369: f64, t2725: f64, t2812: f64, t2813: f64, t2814: f64, t297: f64, t312: f64, t8198: f64, t8217: f64, t894: f64, t940: f64) -> (f64, f64) {
    let t25377 = t8254 * t8246;
    let t25379 = t8251 * t8243;
    let t25381 = t2723 * t7244;
    let t25385 = 0.36282051390366161644e7_f64 * t25335 * t8217 - 0.60470085650610269407e6_f64 * t25338 * t8198 + 0.35163949364965747848e4_f64 * t25343 - 0.17581974682482873924e4_f64 * t25346 + 0.69310201356862480534e2_f64 * t2812 * t10935 * t25348 + 0.519826510176468604e2_f64 * t25353 - 0.41296608323992124631e2_f64 * t25357 + 0.44430618325890501511e2_f64 * t25361 * t2725 + 0.1559479530529405812e2_f64 * t2812 * t2813 * t25365 + 0.15802725909364645561e4_f64 * t25369 * t2814 + 0.5848048239485271795e1_f64 * t940 * t894 * t312 * t24978 * t297 - 0.99111859977581099115e3_f64 * t25377 + 0.33037286659193699704e3_f64 * t25379 + 0.1559479530529405812e2_f64 * t2812 * t2813 * t25381;
    (t25381, t25385)
}
