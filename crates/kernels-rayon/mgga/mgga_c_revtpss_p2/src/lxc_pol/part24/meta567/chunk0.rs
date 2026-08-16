//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1732/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732(t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64, t43776: f64, t87145: f64) -> (f64, f64) {
    let t89857 = 0.55555555555555555555e-1_f64 * t89824 - 0.19999999999999999999e0_f64 * t89828 - 0.24691358024691358025e-1_f64 * t89832 + 0.22222222222222222222e-1_f64 * t81156 - 0.66666666666666666668e-1_f64 * t81158 + 0.22222222222222222222e-1_f64 * t68255 - 0.16666666666666666666e-1_f64 * t89839 - 0.22222222222222222222e-1_f64 * t89843 + 0.3e0_f64 * t89847 + 0.50000000000000000001e-1_f64 * t89851 + 0.66666666666666666668e-1_f64 * t89855;
    let t89863 = t43776 * t87145;
    (t89857, t89863)
}
