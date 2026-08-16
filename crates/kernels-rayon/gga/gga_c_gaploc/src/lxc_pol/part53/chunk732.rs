//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 732/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk732(t13073: f64, t13079: f64, t13113: f64, t13114: f64, t13115: f64, t13116: f64, t13117: f64, t13120: f64, t13886: f64, t13890: f64, t13893: f64, t13895: f64) -> f64 {
    let t14511 = -t13886 - t13890 - 0.29792074959875355558e-1_f64 * t13893 + 0.29792074959875355558e-1_f64 * t13895 - 0.89376224879626066674e-1_f64 * t13073 + t13079 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120;
    t14511
}
