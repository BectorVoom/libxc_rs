//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1199/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1199(t1962: f64, t4537: f64, t4343: f64, t119765: f64, t119779: f64, t126043: f64, t126049: f64, t126052: f64, t126055: f64, t126062: f64, t126065: f64, t126068: f64, t126072: f64, t126076: f64) -> (f64, f64, f64) {
    let t127593 = t1962 * t4537;
    let t127596 = t1962 * t4343;
    let t127615 = -0.34708173928447610099e-2_f64 * t126043 - t119765 + 0.225875734067843736e-2_f64 * t126049 - 0.29749863367240808656e-2_f64 * t126052 - 0.22312397525430606492e-2_f64 * t126055 - t119779 - 0.22312397525430606492e-2_f64 * t126062 - 0.29749863367240808656e-2_f64 * t126065 + 0.7437465841810202164e-3_f64 * t126068 - 0.14874931683620404328e-2_f64 * t126072 - 0.14874931683620404328e-2_f64 * t126076;
    (t127593, t127596, t127615)
}
