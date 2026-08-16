//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1272/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1272(t18432: f64, t1881: f64, t2233: f64, t28313: f64, t28322: f64, t28883: f64, t28886: f64, t446: f64, t5407: f64, t637: f64, t8130: f64, t8255: f64, t92165: f64, t92168: f64, t92170: f64, t92339: f64, t92344: f64, t92351: f64, t93826: f64) -> f64 {
    let t101823 = t8130 * t28886 / 8.0_f64 + t8130 * t28883 / 8.0_f64 - t92165 + t93826 + t92168 + t92170 + t92339 - t2233 * t18432 * t637 / 16.0_f64 + t1881 * t28313 / 8.0_f64 + t1881 * t28322 / 8.0_f64 + t92344 - t92351 - t446 * t5407 * t8255 / 8.0_f64;
    t101823
}
