//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta531<F: Float>(t2453: F, t555: F, t25898: F, t1399: F, t2438: F, t25304: F, t2482: F, t7262: F, t814: F, t820: F, t844: F, t596: F, t7269: F) -> (F, F, F, F, F, F, F, F) {
        let (t94382, t94383, t94386, t94390, t94391, t94423, t94429, t94443) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1836::<F>(t2453, t555, t25898, t1399, t2438, t25304, t2482, t7262, t814, t820, t844, t596, t7269);
    (t94382, t94383, t94386, t94390, t94391, t94423, t94429, t94443)
}
