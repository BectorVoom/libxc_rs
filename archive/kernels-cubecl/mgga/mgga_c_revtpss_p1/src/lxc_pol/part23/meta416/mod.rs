//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta416<F: Float>(t14586: F, t14786: F, t14791: F, t1559: F, t4433: F, t14785: F, t2652: F, t6030: F, t10858: F, t6024: F, t10816: F, t10824: F, t10826: F, t18456: F, t18459: F, t18462: F, t18466: F, t18471: F, t18475: F, t2745: F, t4362: F) -> (F, F, F, F, F, F, F) {
        let (t18477, t18478, t18481, t18482, t18485, t18487, t18489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1798::<F>(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
    (t18477, t18478, t18481, t18482, t18485, t18487, t18489)
}
