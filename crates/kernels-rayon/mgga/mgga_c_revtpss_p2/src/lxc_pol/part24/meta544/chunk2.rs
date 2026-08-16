//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1608/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608(t39554: f64, t39557: f64, t50205: f64, t50214: f64, t61361: f64, t61367: f64, t61371: f64, t61397: f64, t61400: f64, t61407: f64, t61411: f64, t75974: f64, t75978: f64, t75984: f64, t75998: f64, t76010: f64) -> f64 {
    let t87357 = 0.13170898365871023197e0_f64 * t75974 + t39554 - 0.23417857294518679245e0_f64 * t75978 + t39557 - 0.43902994552903410657e-1_f64 * t61361 + 0.87805989105806821314e-1_f64 * t61367 + 0.69394917116090352835e-2_f64 * t61371 + 0.23417857294518679245e0_f64 * t75984 - 0.12142592671231907757e0_f64 * t50205 - 0.13170898365871023197e0_f64 * t75998 - 0.18505311230957427423e-1_f64 * t50214 - 0.7805952431506226415e-2_f64 * t61397 + 0.7805952431506226415e-2_f64 * t61400 + 0.13878983423218070567e-1_f64 * t61407 - 0.11708928647259339623e0_f64 * t76010 - 0.69394917116090352835e-2_f64 * t61411;
    t87357
}
