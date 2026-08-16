//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1946/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1946<F: Float>(t3936: F, t5674: F, t9810: F, t125: F, t5591: F, t1399: F, t4057: F, t5704: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F) -> (F, F, F, F, F, F, F) {
    let t13967 = t3936 * t5674 * t9810;
    let t13975 = t125 * t5591;
    let t13977 = t3936 * t13975 * t1399;
    let t13981 = t3936 * t5704 * t4057;
    let t13985 = t9818 * t1872 * t1399;
    let t13987 = F::cast_from(0.10164000561857065645e-3_f64) * t9816 * t13985;
    let t13988 = t9962 * t5706;
    (t13967, t13975, t13977, t13981, t13985, t13987, t13988)
}
