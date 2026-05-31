//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1755/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755<F: Float>(t6442: F, t43946: F, t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F) -> (F, F, F) {
    let t90422 = t6442 * t6442;
    let t90423 = t43946 * t90422;
    let t90437 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t89824 - F::cast_from(8.0_f64) * t89828 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t89832 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t81156 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t81158 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68255 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t89839 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t89843 + F::cast_from(12.0_f64) * t89847 + F::cast_from(2.0_f64) * t89851 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t89855;
    (t90422, t90423, t90437)
}
