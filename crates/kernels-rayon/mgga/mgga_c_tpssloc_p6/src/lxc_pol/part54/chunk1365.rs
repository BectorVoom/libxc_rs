//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1365/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1365(t26142: f64, t7042: f64, t25010: f64, t8607: f64, t23938: f64, t7468: f64, t26977: f64, t26003: f64, t31304: f64, t7756: f64, t33553: f64, t652: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121224 = 2.0_f64 * t7042 * t26142;
    let t121226 = t8607 * t25010;
    let t121228 = 2.0_f64 * t23938 * t7468;
    let t121231 = 2.0_f64 * t26977 * t7468;
    let t121233 = 2.0_f64 * t7042 * t26003;
    let t121234 = t31304 * t7756;
    let t121237 = 2.0_f64 * t652 * t33553 * t671;
    (t121224, t121226, t121228, t121231, t121233, t121234, t121237)
}
