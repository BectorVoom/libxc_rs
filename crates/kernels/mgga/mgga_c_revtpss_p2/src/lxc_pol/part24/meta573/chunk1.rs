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
    let t90437 = F::new(20.0) / F::new(9.0) * t89824 - F::new(8.0) * t89828 - F::new(80.0) / F::new(81.0) * t89832 + F::new(8.0) / F::new(9.0) * t81156 - F::new(8.0) / F::new(3.0) * t81158 + F::new(8.0) / F::new(9.0) * t68255 - F::new(2.0) / F::new(3.0) * t89839 - F::new(8.0) / F::new(9.0) * t89843 + F::new(12.0) * t89847 + F::new(2.0) * t89851 + F::new(8.0) / F::new(3.0) * t89855;
    (t90422, t90423, t90437)
}
