//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1192/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1192(t119422: f64, t119424: f64, t121458: f64, t121460: f64, t125208: f64, t127412: f64, t127416: f64, t127421: f64, t1464: f64, t1921: f64, t2045: f64, t28235: f64, t3: f64, t32343: f64, t33984: f64, t575: f64, t5808: f64, t8603: f64) -> f64 {
    let t127425 = t127412 * t3 * t575 + t1464 * t33984 + t1921 * t32343 + 2.0_f64 * t2045 * t28235 + t5808 * t8603 + t119422 + t119424 + 2.0_f64 * t121458 + 2.0_f64 * t121460 + t125208 + t127416 + 2.0_f64 * t127421;
    t127425
}
