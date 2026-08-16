//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1307/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1307(t29978: f64, t3: f64, t112: f64, t8153: f64, t111: f64, t2186: f64, t671: f64, t8143: f64, t2180: f64, t2363: f64, t12521: f64, t12524: f64, t1401: f64, t16535: f64, t2319: f64, t29934: f64, t3938: f64, t3941: f64, t577: f64, t8161: f64, t8166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29979 = t3 * t29978;
    let t29993 = t8153 * t112;
    let t29996 = t2186 * t111;
    let t30009 = t8143 * t671;
    let t30012 = t2180 * t2363;
    let t30017 = 0.45e1_f64 * t29978 * t577 + 27.0_f64 * t29993 * t671 + 27.0_f64 * t29996 * t2319 + 0.135e2_f64 * t8161 * t2363 + 0.135e2_f64 * t12521 * t2180 + 54.0_f64 * t12524 * t8166 + 27.0_f64 * t3938 * t8143 + 27.0_f64 * t16535 * t2180 + 54.0_f64 * t3941 * t30009 + 27.0_f64 * t3941 * t30012 + 0.135e2_f64 * t1401 * t29934;
    (t29979, t29993, t29996, t30009, t30012, t30017)
}
