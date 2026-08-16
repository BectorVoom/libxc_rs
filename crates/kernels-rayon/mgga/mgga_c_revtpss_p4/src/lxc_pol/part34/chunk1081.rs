//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1081/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1081(t24881: f64, t25025: f64, t12587: f64, t1300: f64, t1832: f64, t198: f64, t20692: f64, t24468: f64, t24478: f64, t24482: f64, t24484: f64, t24490: f64, t24492: f64, t24496: f64, t24500: f64, t24501: f64, t24763: f64, t24767: f64, t336: f64, t5023: f64) -> f64 {
    let t25026 = t24881 + t25025;
    let t25030 = 2.0_f64 * t12587 * t198 * t24501 * t336 + t1300 * t198 * t25026 * t336 - 3.0_f64 * t1832 * t20692 * t5023 - t24468 - t24478 - t24482 - t24484 + t24490 - t24492 + t24496 - t24500 + t24763 - t24767;
    t25030
}
