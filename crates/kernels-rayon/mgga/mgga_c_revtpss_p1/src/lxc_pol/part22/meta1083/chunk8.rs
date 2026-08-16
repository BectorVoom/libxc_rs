//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3921/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3921(t21969: f64, t566: f64, t1353: f64, t13600: f64, t22486: f64, t3889: f64, t39989: f64, t4139: f64, t47086: f64, t47088: f64, t5536: f64, t5591: f64, t6836: f64, t74121: f64, t74122: f64, t74123: f64, t74124: f64, t74125: f64, t9599: f64) -> f64 {
    let t75379 = t566 * t21969;
    let t75386 = 12.0_f64 * t1353 * t5536 * t75379 + 12.0_f64 * t13600 * t4139 * t5591 + 6.0_f64 * t22486 * t3889 * t5536 - 6.0_f64 * t5536 * t6836 * t9599 - t39989 - t47086 + t47088 - t74121 + t74122 + t74123 + t74124 - t74125;
    t75386
}
