//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 987/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk987(t2383: f64, t3689: f64, t10575: f64, t10581: f64, t10587: f64, t10592: f64, t10596: f64, t10600: f64, t10602: f64, t10606: f64, t10610: f64, t10614: f64, t10617: f64, t2173: f64, t3626: f64) -> f64 {
    let t10620 = 7.0_f64 / 576.0_f64 * t2383 * t3689;
    let t10621 = -5.0_f64 / 384.0_f64 * t2173 * t10575 + t2173 * t10581 / 384.0_f64 - t3626 * t10587 / 192.0_f64 - t2173 * t10592 / 1536.0_f64 - t2173 * t10596 / 3072.0_f64 + t10600 + t2173 * t10602 / 384.0_f64 + t2173 * t10606 / 768.0_f64 + t3626 * t10610 / 768.0_f64 + t3626 * t10614 / 1536.0_f64 - 119.0_f64 / 3456.0_f64 * t10617 + t10620;
    t10621
}
