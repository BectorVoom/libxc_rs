//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2160/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2160(t92963: f64, t92966: f64, t92969: f64, t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64, t14853: f64, t7045: f64, t14857: f64, t25234: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98960 = 0.10164000561857065645e-4_f64 * t92963;
    let t98961 = 0.72286371995927450868e-4_f64 * t92966;
    let t98962 = 35.0_f64 / 108.0_f64 * t92969;
    let t98964 = t9775 * t27253;
    let t98968 = t2661 * t7043 * t240 * t14833;
    let t98970 = t7045 * t14853;
    let t98972 = t25234 * t14857;
    (t98960, t98961, t98962, t98964, t98968, t98970, t98972)
}
