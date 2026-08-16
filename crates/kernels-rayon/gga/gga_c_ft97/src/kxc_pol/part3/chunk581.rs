//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 581/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk581(t4518: f64, t582: f64, t4522: f64, t2112: f64, t24: f64, t4668: f64, t4714: f64, t586: f64, t2092: f64, t3497: f64, t3513: f64, t462: f64, t4759: f64, t4762: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t4765 = t582 * t4518;
    let t4768 = t582 * t4522;
    let t4772 = t24 * t2112 * t4668;
    let t4776 = t24 * t586 * t4714;
    let t4778 = t2092 + 2.0_f64 / 9.0_f64 * t3497 + 2.0_f64 / 3.0_f64 * t3513 - 2.0_f64 / 9.0_f64 * t462 * t4759 + 2.0_f64 / 3.0_f64 * t462 * t4762 + 2.0_f64 / 3.0_f64 * t462 * t4765 - t462 * t4768 / 3.0_f64 + 2.0_f64 * t92 * t4772 - t92 * t4776;
    (t4765, t4768, t4772, t4776, t4778)
}
