//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 823/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk823(t13140: f64, t825: f64, t1114: f64, t8520: f64, t12332: f64, t12333: f64, t12334: f64, t13070: f64, t13071: f64, t4341: f64, t4349: f64, t4499: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64) -> (f64, f64, f64) {
    let t13141 = t13140 * t825;
    let t13142 = t1114 * t13141;
    let t13148 = 0.18981728898494541632e1_f64 * t8520;
    let t13149 = -t13070 - t12332 + t13071 + t4341 - t4349 - t4499 + t4503 - t4506 - t4513 + t4539 + t4542 + t12333 - t13148 + t12334;
    (t13141, t13142, t13149)
}
