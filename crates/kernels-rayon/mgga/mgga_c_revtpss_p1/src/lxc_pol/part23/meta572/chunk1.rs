//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2168/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2168(t1907: f64, t6781: f64, t1868: f64, t198: f64, t21937: f64, t22466: f64, t22928: f64, t22929: f64, t22930: f64, t22931: f64, t22932: f64, t4139: f64, t532: f64, t5532: f64, t6816: f64, t9542: f64, t9593: f64, t9598: f64, t9854: f64, t9857: f64, t9865: f64, t9868: f64) -> (f64, f64) {
    let t23087 = t6781 * t1907;
    let t23092 = 2.0_f64 * t198 * t23087 * t532 * t9593 + 9.0_f64 * t1868 * t21937 * t4139 - 9.0_f64 * t1868 * t22466 * t4139 + 9.0_f64 * t4139 * t5532 * t6816 - t22928 + t22929 + t22930 + t22931 + t22932 + t9542 + t9598 - t9854 - t9857 + t9865 + t9868;
    (t23087, t23092)
}
