//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2101/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2101(t13941: f64, t94423: f64, t14005: f64, t13834: f64, t27940: f64, t13841: f64, t26028: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t48662: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98185 = t94423 * t13941;
    let t98186 = 0.2032800112371413129e-3_f64 * t98185;
    let t98187 = t94423 * t14005;
    let t98188 = 0.50820002809285328226e-4_f64 * t98187;
    let t98189 = t27940 * t13834;
    let t98191 = t26028 * t13841;
    let t98193 = t94429 * t5706;
    let t98194 = 0.16006300097412701803e-1_f64 * t98193;
    let t98196 = t1941 * t9817;
    let t98197 = t98196 * t48662;
    (t98186, t98188, t98189, t98191, t98194, t98197)
}
