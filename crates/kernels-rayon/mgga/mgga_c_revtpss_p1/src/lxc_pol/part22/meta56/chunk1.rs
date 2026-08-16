//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 408/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk408(t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t342: f64, t381: f64, t989: f64) -> f64 {
    let t1096 = 0.65854491829355115987e0_f64 * t989 * t381 - 0.65854491829355115987e0_f64 * t1024 * t1083 + 0.65854491829355115987e0_f64 * t1087 * t1090 + 0.65854491829355115987e0_f64 * t342 * t1093;
    t1096
}
