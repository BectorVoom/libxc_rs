//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta339<F: Float>(t2371: F, t93: F, t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t10208: F, t1513: F, t2340: F, t2339: F, t4287: F, t665: F, t2366: F, t4263: F, t10227: F, t1504: F, t2350: F, t2349: F, t97: F, t2255: F, t658: F, t2256: F, t4269: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13440, t13448, t13451, t13453, t13455, t13458) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1139::<F>(t2371, t93, t1514, t2289, t4264, t625, t4288, t10208, t1513, t2340, t2339, t4287);
        let (t13459, t13462, t13472, t13475, t13476, t13479) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1140::<F>(t13458, t665, t2366, t4263, t10227, t1504, t2350, t2349, t97, t2255, t658, t2256, t4269);
    (t13440, t13448, t13451, t13453, t13455, t13459, t13462, t13472, t13475, t13476, t13479)
}
