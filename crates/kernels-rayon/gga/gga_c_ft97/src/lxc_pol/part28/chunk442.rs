//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 442/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk442(t2179: f64, t6708: f64, t144: f64, t5962: f64, t5965: f64, t6659: f64, t6663: f64, t6667: f64, t6671: f64, t6675: f64, t6679: f64, t6683: f64) -> (f64, f64) {
    let t6709 = t2179 * t6708;
    let t6710 = t144 * t6709;
    let t6718 = t6659 / 4.0_f64 + t5962 + t6663 / 6.0_f64 + t6667 - t6671 / 2.0_f64 + t5965 + t6675 / 3.0_f64 + 2.0_f64 * t6679 - t6683;
    (t6710, t6718)
}
