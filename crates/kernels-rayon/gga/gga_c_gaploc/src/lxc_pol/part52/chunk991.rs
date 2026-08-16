//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 991/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk991(t14440: f64, t747: f64, t12035: f64, t8045: f64, t12270: f64, t12277: f64, t1960: f64, t2969: f64, t2972: f64, t3073: f64, t331: f64, t3749: f64, t38892: f64, t45974: f64, t45976: f64, t45983: f64, t45988: f64, t45990: f64, t45992: f64, t45993: f64, t45997: f64, t49980: f64, t49983: f64, t50338: f64, t50356: f64, t50373: f64, t50407: f64, t50421: f64, t50435: f64, t50454: f64, t50465: f64, t841: f64, t8440: f64) -> (f64, f64) {
    let t50470 = t14440 * t747;
    let t50475 = 4.0_f64 * t8045 * t12035;
    let t50478 = -2.0_f64 * t8440 * t3749 - t49980 + 4.0_f64 * t1960 * t3073 * t3749 + t49983 + t45974 + t45976 - 2.0_f64 * t2969 * t12270 + (t50338 + t50356 + t50373 + t50407 + t50421 + t50435 + t50454 + t50465) * t331 + t45983 - t50470 * t841 - 2.0_f64 * t12277 * t3073 + t45988 + t45990 - t45992 - t45993 - t45997 - t50475 + 4.0_f64 * t38892 * t2972;
    (t50475, t50478)
}
