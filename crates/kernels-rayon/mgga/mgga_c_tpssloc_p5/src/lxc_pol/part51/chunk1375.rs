//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1375/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1375(t26977: f64, t7468: f64, t26003: f64, t7042: f64, t31304: f64, t7756: f64, t33553: f64, t652: f64, t671: f64, t4072: f64, t8595: f64, t1983: f64, t27144: f64, t8643: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121231 = 2.0_f64 * t26977 * t7468;
    let t121233 = 2.0_f64 * t7042 * t26003;
    let t121234 = t31304 * t7756;
    let t121237 = 2.0_f64 * t652 * t33553 * t671;
    let t121240 = 2.0_f64 * t652 * t8595 * t4072;
    let t121253 = t1983 * t27144 * t8643;
    (t121231, t121233, t121234, t121237, t121240, t121253)
}
