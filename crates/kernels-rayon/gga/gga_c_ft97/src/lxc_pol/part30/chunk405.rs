//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 405/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk405(t1690: f64, t6790: f64, t6793: f64, t1127: f64, t6027: f64, t1701: f64, t1096: f64, t6036: f64) -> (f64, f64, f64, f64) {
    let t6795 = t1690 * t6790 * t6793;
    let t6798 = t6027 * t1127;
    let t6799 = t1701 * t6798;
    let t6804 = t6036 * t1096;
    (t6795, t6798, t6799, t6804)
}
