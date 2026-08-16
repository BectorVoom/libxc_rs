//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1880/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880(t27349: f64, t689: f64, t25260: f64, t4368: f64, t820: f64, t844: f64, t4462: f64, t92951: f64, t92963: f64, t92966: f64, t92969: f64, t27253: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98892 = t27349 * t689;
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98949 = t92951 * t4462;
    let t98960 = 0.10164000561857065645e-4_f64 * t92963;
    let t98961 = 0.72286371995927450868e-4_f64 * t92966;
    let t98962 = 35.0_f64 / 108.0_f64 * t92969;
    let t98964 = t9775 * t27253;
    (t98892, t98937, t98949, t98960, t98961, t98962, t98964)
}
