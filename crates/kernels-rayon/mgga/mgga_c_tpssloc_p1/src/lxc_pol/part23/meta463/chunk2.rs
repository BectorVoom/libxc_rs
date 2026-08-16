//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1356/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356(t10214: f64, t10217: f64, t10278: f64, t1597: f64, t21444: f64, t2979: f64, t2980: f64, t343: f64, t42976: f64, t4546: f64, t48336: f64, t48397: f64, t61408: f64, t61489: f64, t61597: f64, t61600: f64, t69796: f64, t69801: f64, t69806: f64, t75836: f64, t75847: f64, t973: f64, t977: f64) -> f64 {
    let t76974 = 0.22222222222222222221e-2_f64 * t69796 - 0.33333333333333333332e-2_f64 * t69801 + 0.11522633744855967078e-2_f64 * t69806 - 0.1037037037037037037e-1_f64 * t973 * t10214 * t42976 * t75836 - 0.33333333333333333332e-2_f64 * t973 * t4546 * t21444 * t1597 * t343 + 0.13333333333333333332e-1_f64 * t973 * t2979 * t10217 * t75836 + 0.11111111111111111111e-2_f64 * t973 * t2979 * t2980 * t75847 - 0.66666666666666666664e-2_f64 * t973 * t977 * t10278 * t75836 + 0.74074074074074074072e-3_f64 * t61408 - 0.12345679012345679012e-2_f64 * t48336 + 0.74074074074074074072e-3_f64 * t61489 - 0.37037037037037037036e-3_f64 * t61597 - 0.49382716049382716048e-3_f64 * t61600 + 0.41152263374485596707e-3_f64 * t48397;
    t76974
}
