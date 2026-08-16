//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 956/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk956(t1453: f64, t32101: f64, t32102: f64, t32107: f64, t32109: f64, t32112: f64, t32116: f64, t32823: f64, t32824: f64, t32840: f64, t32843: f64, t32845: f64, t32849: f64, t32850: f64, t33343: f64, t33346: f64, t33381: f64, t569: f64, t651: f64, t671: f64, t7586: f64, t7591: f64, t8463: f64, t8967: f64) -> f64 {
    let t33384 = t1453 * t8967 - 2.0_f64 * t33343 * t651 - 2.0_f64 * t33346 * t671 + t33381 * t569 - 4.0_f64 * t7586 * t7591 + t32101 - t32102 - t32107 - t32109 - t32112 - t32116 + 2.0_f64 * t32823 + 2.0_f64 * t32824 - 4.0_f64 * t32840 - 4.0_f64 * t32843 - 4.0_f64 * t32845 - 2.0_f64 * t32849 + 6.0_f64 * t32850 - t8463;
    t33384
}
