//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1850/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850(t24949: f64, t24953: f64, t3: f64, t112: f64, t7415: f64, t111: f64, t2169: f64, t2319: f64, t2363: f64, t23886: f64, t23888: f64, t23890: f64, t23892: f64, t23895: f64, t23898: f64, t23900: f64, t577: f64, t671: f64, t7423: f64) -> (f64, f64, f64, f64, f64) {
    let t24954 = t24949 + t24953;
    let t24955 = t3 * t24954;
    let t24969 = t7415 * t112;
    let t24972 = t2169 * t111;
    let t24977 = 0.45e1_f64 * t24954 * t577 + 27.0_f64 * t24969 * t671 + 27.0_f64 * t24972 * t2319 + 0.135e2_f64 * t7423 * t2363 + t23886 + t23888 + t23890 + t23892 + t23895 + t23898 + t23900;
    (t24954, t24955, t24969, t24972, t24977)
}
