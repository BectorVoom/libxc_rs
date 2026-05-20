//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1888;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta551<F: Float>(t7289: F, t96370: F, t7284: F, t96282: F, t94669: F, t96271: F, t26277: F, t94913: F, t25944: F, t96259: F, t1385: F, t7506: F, t10073: F, t25937: F, t7282: F, t26069: F, t96255: F, t2453: F, t3908: F, t7507: F, t2435: F, t26301: F, t96276: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96371, t96374, t96378, t96380, t96382, t96392) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1888::<F>(t7289, t96370, t7284, t96282, t94669, t96271, t26277, t94913, t25944, t96259, t1385, t7506);
        let (t96398, t96401, t96403, t96410, t96412) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889::<F>(t10073, t25937, t7282, t7506, t26069, t96255, t2453, t3908, t7507, t2435, t26301, t7289, t96276);
    (t96371, t96374, t96378, t96380, t96382, t96392, t96398, t96401, t96403, t96410, t96412)
}
