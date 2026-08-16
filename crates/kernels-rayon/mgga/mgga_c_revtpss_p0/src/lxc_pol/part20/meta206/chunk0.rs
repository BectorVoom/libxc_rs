//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 978/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk978(t10227: f64, t10228: f64, t2349: f64, t658: f64, t2256: f64, t9343: f64, t100: f64, t106: f64, t107: f64, t2358: f64, t661: f64, t2357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10229 = t10227 * t10228;
    let t10232 = t2349 * t658;
    let t10233 = t10232 * t2256;
    let t10236 = 3.0_f64 * t9343;
    let t10237 = t100 * t10236;
    let t10240 = t107 * t106;
    let t10241 = 1.0_f64 / t10240;
    let t10242 = t2358 * t661;
    let t10243 = t10241 * t10242;
    let t10246 = t2357 * t661;
    (t10229, t10232, t10233, t10236, t10237, t10241, t10243, t10246)
}
