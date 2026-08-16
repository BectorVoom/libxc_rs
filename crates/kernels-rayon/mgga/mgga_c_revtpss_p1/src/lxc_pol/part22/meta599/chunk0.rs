//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2485/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2485(t3022: f64, t6219: f64, t6223: f64, t2986: f64, t6205: f64, t974: f64, t981: f64, t4708: f64, t4724: f64, t3336: f64, t6396: f64, t6184: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19143 = 0.11696447245269292414e1_f64 * t3022 * t6219;
    let t19145 = 0.5848223622634646207e0_f64 * t3022 * t6223;
    let t19146 = t2986 * t6205;
    let t19147 = t19146 * t974;
    let t19149 = 0.11696447245269292414e1_f64 * t981 * t19147;
    let t19150 = t4724 * t4708;
    let t19152 = 0.23392894490538584828e1_f64 * t981 * t19150;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    (t19143, t19145, t19146, t19147, t19149, t19150, t19152, t19153, t19156)
}
