//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1170/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1170(t1358: f64, t2339: f64, t25761: f64, t6536: f64, t7888: f64, t2754: f64, t874: f64) -> (f64, f64, f64) {
    let t31581 = 0.18970004423784099733e-1_f64 * t1358 * t25761 * t2339;
    let t31584 = 0.18970004423784099733e-1_f64 * t1358 * t7888 * t6536;
    let t31585 = t2754 * t874;
    (t31581, t31584, t31585)
}
