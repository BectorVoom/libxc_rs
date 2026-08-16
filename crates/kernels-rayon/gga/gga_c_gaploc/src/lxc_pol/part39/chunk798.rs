//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 798/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk798(t13066: f64, t13070: f64, t13074: f64, t13079: f64, t13114: f64, t13115: f64, t13116: f64, t13120: f64, t13890: f64, t13893: f64, t13895: f64, t13898: f64, t13899: f64, t13901: f64) -> f64 {
    let t13903 = -0.19171462976960374838e0_f64 * t13066 - t13890 - 0.14896037479937677779e-1_f64 * t13893 + 0.14896037479937677779e-1_f64 * t13895 + 0.19171462976960374838e0_f64 * t13070 - t13074 + t13079 - t13898 - t13114 + t13115 + t13116 + t13899 + t13120 + 0.71500979903700853338e0_f64 * t13901;
    t13903
}
