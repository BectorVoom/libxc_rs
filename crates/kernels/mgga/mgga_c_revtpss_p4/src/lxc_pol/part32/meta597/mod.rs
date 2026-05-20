//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta597<F: Float>(t2435: F, t28448: F, t28314: F, t93364: F, t103431: F, t25375: F, t212: F, t28340: F, t689: F, t780: F, t103182: F, t93281: F, t103421: F, t7058: F, t11064: F, t8019: F, t28993: F, t571: F, t2118: F, t5789: F, t1464: F, t8113: F, t1913: F, t7560: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t103490, t103494, t103521, t103529, t103543) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930::<F>(t2435, t28448, t28314, t93364, t103431, t25375, t212, t28340, t689, t780, t103182, t93281);
        let (t103547, t103586, t104062, t104071, t104073, t104077) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1931::<F>(t103421, t7058, t11064, t8019, t28993, t571, t2118, t5789, t1464, t8113, t1913, t7560);
    (t103490, t103494, t103521, t103529, t103543, t103547, t103586, t104062, t104071, t104073, t104077)
}
