//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1162/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1162(t225: f64, t23185: f64, t23187: f64, t23192: f64, t23224: f64, t10626: f64, t23114: f64, t4416: f64, t5962: f64, t23148: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t4415: f64, t6006: f64, t6010: f64, t6013: f64) -> (f64, f64, f64, f64, f64) {
    let t23227 = (t23185 + t23187 + t23192 + t23224) * t225;
    let t23235 = t10626 * t23114;
    let t23238 = t4416 * t5962;
    let t23241 = t832 * t23148;
    let t23244 = -36.0_f64 * t1553 * t6010 + 9.0_f64 * t1553 * t6013 + 9.0_f64 * t1555 * t6006 + 60.0_f64 * t227 * t23235 + 3.0_f64 * t227 * t23241 - t229 * t23227 - 36.0_f64 * t23238 * t4415;
    (t23227, t23235, t23238, t23241, t23244)
}
