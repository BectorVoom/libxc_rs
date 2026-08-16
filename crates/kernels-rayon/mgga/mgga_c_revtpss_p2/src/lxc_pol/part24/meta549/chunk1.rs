//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1623/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623(t1553: f64, t1555: f64, t18592: f64, t18599: f64, t225: f64, t227: f64, t229: f64, t23148: f64, t23227: f64, t23235: f64, t23238: f64, t23241: f64, t2638: f64, t40231: f64, t4415: f64, t4416: f64, t5962: f64, t6006: f64, t6010: f64, t6013: f64, t832: f64, t87543: f64, t87548: f64, t87553: f64, t87634: f64, t87635: f64, t87637: f64, t87645: f64, t87652: f64, t87664: f64, t87672: f64, t87680: f64) -> f64 {
    let t87713 = -(t87634 + t87635 + t87637 + t87645 + t87652 + t87664 + t87672 + t87680) * t225 * t229 + 12.0_f64 * t23227 * t1555 - 72.0_f64 * t6006 * t6010 + 18.0_f64 * t6006 * t6013 + 240.0_f64 * t1553 * t23235 - 144.0_f64 * t18592 * t23238 + 12.0_f64 * t1553 * t23241 - 360.0_f64 * t227 * t40231 * t87553 + 360.0_f64 * t4415 * t18599 * t5962 - 36.0_f64 * t227 * t2638 * t87548 - 48.0_f64 * t4415 * t4416 * t23148 + 3.0_f64 * t227 * t832 * t87543;
    t87713
}
