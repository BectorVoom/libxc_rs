//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2065/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2065(t25260: f64, t4368: f64, t820: f64, t844: f64, t4462: f64, t92951: f64, t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64) -> (f64, f64, f64, f64) {
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98949 = t92951 * t4462;
    let t98950 = 0.16006300097412701803e-1_f64 * t98949;
    let t98964 = t9775 * t27253;
    let t98968 = t2661 * t7043 * t240 * t14833;
    (t98937, t98950, t98964, t98968)
}
