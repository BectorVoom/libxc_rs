//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1925/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1925(t6722: f64, t7573: f64, t3: f64, t3966: f64, t1933: f64, t4603: f64, t6717: f64, t1597: f64, t1934: f64, t1025: f64, t1046: f64, t1607: f64, t1618: f64, t1920: f64, t1937: f64, t23419: f64, t23422: f64, t23425: f64, t23437: f64, t25571: f64, t25574: f64, t25577: f64, t25580: f64, t4575: f64, t4579: f64, t6735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25585 = t6722 * t7573;
    let t25588 = t3 * t3966;
    let t25589 = t1933 * t25588;
    let t25598 = t6717 * t4603;
    let t25600 = t1934 * t1597;
    let t25601 = t1933 * t25600;
    let t25605 = -t1920 * t25571 / 144.0_f64 + t1920 * t25574 / 216.0_f64 + t25577 * t1025 / 1536.0_f64 + t25580 * t1046 / 2304.0_f64 - t23437 * t1618 / 288.0_f64 - 0.80745512188280781712e-3_f64 * t25585 * t1937 + 0.10093189023535097714e-3_f64 * t25589 * t1937 + t23419 * t4575 / 2304.0_f64 + t23419 * t4579 / 2304.0_f64 - t23422 * t1607 / 108.0_f64 + t25598 / 864.0_f64 - 0.10093189023535097714e-3_f64 * t25601 * t6735 + t23425 / 864.0_f64;
    (t25585, t25588, t25589, t25600, t25601, t25605)
}
