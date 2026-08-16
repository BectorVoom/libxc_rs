//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1888;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta551(t7289: f64, t96370: f64, t7284: f64, t96282: f64, t94669: f64, t96271: f64, t26277: f64, t94913: f64, t25944: f64, t96259: f64, t1385: f64, t7506: f64, t10073: f64, t25937: f64, t7282: f64, t26069: f64, t96255: f64, t2453: f64, t3908: f64, t7507: f64, t2435: f64, t26301: f64, t96276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96371, t96374, t96378, t96380, t96382, t96392) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1888(t7289, t96370, t7284, t96282, t94669, t96271, t26277, t94913, t25944, t96259, t1385, t7506);
        let (t96398, t96401, t96403, t96410, t96412) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889(t10073, t25937, t7282, t7506, t26069, t96255, t2453, t3908, t7507, t2435, t26301, t7289, t96276);
    (t96371, t96374, t96378, t96380, t96382, t96392, t96398, t96401, t96403, t96410, t96412)
}
