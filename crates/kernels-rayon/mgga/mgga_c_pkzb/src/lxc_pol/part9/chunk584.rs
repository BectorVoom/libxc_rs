//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 584/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk584(t133: f64, t2387: f64, t945: f64, t2393: f64, t410: f64, t2126: f64, t394: f64, t2434: f64, t2421: f64, t2433: f64, t2436: f64, t2439: f64, t397: f64, t943: f64, t946: f64) -> (f64, f64, f64, f64) {
    let t2442 = t2387 * t133;
    let t2443 = t2442 * t945;
    let t2446 = t2393 * t410;
    let t2447 = t2126 * t394;
    let t2448 = t2434 * t2447;
    let t2453 = 0.13170898365871023197e1_f64 * t2433 * t2436 + 0.13170898365871023197e1_f64 * t2439 * t946 + 0.65854491829355115987e0_f64 * t943 * t2443 - 0.65854491829355115987e0_f64 * t2446 * t2448 + 0.65854491829355115987e0_f64 * t397 * t2421;
    (t2442, t2443, t2448, t2453)
}
