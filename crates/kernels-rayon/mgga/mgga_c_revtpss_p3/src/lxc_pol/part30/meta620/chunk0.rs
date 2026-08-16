//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2132/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2132(t1940: f64, t2255: f64, t7087: f64, t27383: f64, t61155: f64, t27375: f64, t92790: f64, t14767: f64, t27159: f64, t4537: f64, t605: f64, t15071: f64, t30: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98684 = 2.0_f64 * t1940 * t7087 * t2255;
    let t98688 = t27383 * t61155;
    let t98694 = t92790 * t27375;
    let t98699 = t27159 * t14767;
    let t98702 = t605 * t4537;
    let t98705 = t30 * t15071;
    (t98684, t98688, t98694, t98699, t98702, t98705)
}
