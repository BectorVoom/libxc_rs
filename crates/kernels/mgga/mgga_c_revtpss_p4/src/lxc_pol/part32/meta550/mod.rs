//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1865;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta550<F: Float>(t10073: F, t25937: F, t7282: F, t7506: F, t26069: F, t96255: F, t2453: F, t3908: F, t7507: F, t2435: F, t26301: F, t7289: F, t96276: F, t94589: F, t96279: F, t26265: F, t9686: F, t2098: F, t4075: F, t786: F, t2103: F, t47567: F, t26261: F, t40270: F, t25920: F, t26260: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96398, t96401, t96403, t96410, t96412) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1865::<F>(t10073, t25937, t7282, t7506, t26069, t96255, t2453, t3908, t7507, t2435, t26301, t7289, t96276);
        let (t96456, t96460, t96463, t96473, t96491, t96503) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1866::<F>(t94589, t96279, t26265, t9686, t2098, t4075, t786, t2103, t47567, t26261, t40270, t10073, t25920, t26260);
    (t96398, t96401, t96403, t96410, t96412, t96456, t96460, t96463, t96473, t96491, t96503)
}
