//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 704/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk704(t20022: f64, t8314: f64, t1780: f64, t20031: f64, t3127: f64, t1787: f64, t20141: f64, t20334: f64, t20337: f64, t20341: f64, t20345: f64, t20349: f64, t20353: f64, t20356: f64, t20359: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t20362 = t8314 * t20022;
    let t20363 = t1780 * t20362;
    let t20366 = t3127 * t20031;
    let t20369 = t1787 * t20141;
    let t20371 = -t92 * t20334 - t462 * t20337 / 3.0_f64 - 6.0_f64 * t92 * t20341 + 6.0_f64 * t462 * t20345 - 10.0_f64 / 27.0_f64 * t462 * t20349 - 2.0_f64 * t462 * t20353 + 2.0_f64 * t462 * t20356 + 2.0_f64 / 3.0_f64 * t462 * t20359 + 4.0_f64 / 3.0_f64 * t462 * t20363 - 2.0_f64 / 3.0_f64 * t462 * t20366 + t462 * t20369;
    (t20362, t20363, t20366, t20369, t20371)
}
