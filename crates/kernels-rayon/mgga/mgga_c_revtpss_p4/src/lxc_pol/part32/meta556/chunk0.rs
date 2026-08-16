//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1875/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1875(t13985: f64, t94423: f64, t13878: f64, t25972: f64, t94479: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64, t25978: f64, t5629: f64, t1885: f64, t94459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98202 = t94423 * t13985;
    let t98206 = t25972 * t13878;
    let t98217 = 0.4065600224742826258e-4_f64 * t94479;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98222 = t25978 * t5629;
    let t98224 = t94459 * t1885;
    (t98202, t98206, t98217, t98218, t98220, t98222, t98224)
}
