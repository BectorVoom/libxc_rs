//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1732/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732<F: Float>(t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t43776: F, t87145: F) -> (F, F) {
    let t89857 = F::cast_from(0.55555555555555555555e-1_f64) * t89824 - F::cast_from(0.19999999999999999999e0_f64) * t89828 - F::cast_from(0.24691358024691358025e-1_f64) * t89832 + F::cast_from(0.22222222222222222222e-1_f64) * t81156 - F::cast_from(0.66666666666666666668e-1_f64) * t81158 + F::cast_from(0.22222222222222222222e-1_f64) * t68255 - F::cast_from(0.16666666666666666666e-1_f64) * t89839 - F::cast_from(0.22222222222222222222e-1_f64) * t89843 + F::new(0.3e0) * t89847 + F::cast_from(0.50000000000000000001e-1_f64) * t89851 + F::cast_from(0.66666666666666666668e-1_f64) * t89855;
    let t89863 = t43776 * t87145;
    (t89857, t89863)
}
