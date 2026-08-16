//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1111/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1111(t11602: f64, t11652: f64, t11657: f64, t11665: f64, t11668: f64, t11672: f64, t19400: f64, t19422: f64, t19423: f64, t19425: f64, t19431: f64, t19432: f64, t19433: f64, t19434: f64, t19435: f64, t19436: f64, t19437: f64, t19441: f64) -> f64 {
    let t19912 = -t19400 - t11602 - t11652 - t19422 + t11657 + t19423 + t19425 + t11665 + t11668 - t11672 + t19431 + t19432 + t19433 + t19434 + t19435 + t19436 - t19437 - t19441;
    t19912
}
