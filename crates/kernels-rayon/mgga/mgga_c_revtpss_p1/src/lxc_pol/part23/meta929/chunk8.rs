//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3041/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3041(t19137: f64, t19153: f64, t27717: f64, t5019: f64, t5023: f64, t63907: f64, t78339: f64, t78342: f64, t78703: f64, t78706: f64, t78709: f64, t78712: f64, t78715: f64, t78717: f64) -> f64 {
    let t81088 = 6.0_f64 * t19137 * t5019 * t5023 - 3.0_f64 * t19153 * t5019 * t5023 + 6.0_f64 * t27717 * t5023 * t63907 + t78339 + t78342 - t78703 - t78706 + t78709 - t78712 + t78715 - t78717;
    t81088
}
