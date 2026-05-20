//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1896;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta555<F: Float>(t3917: F, t96576: F, t94701: F, t96204: F, t25878: F, t96242: F, t26359: F, t9303: F, t2118: F, t4153: F, t116: F, t26153: F, t1353: F, t28198: F, t13790: F, t4102: F, t685: F, t72: F, t1444: F, t5740: F, t675: F, t14109: F, t25900: F, t1892: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t96577, t96584, t96588, t96591, t96633, t96640) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1896::<F>(t3917, t96576, t94701, t96204, t25878, t96242, t26359, t9303, t2118, t4153, t116, t26153);
        let (t97654, t97680, t97685, t97688, t97699) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1897::<F>(t1353, t28198, t13790, t4102, t685, t72, t1444, t5740, t675, t14109, t25900, t1892, t786);
    (t96577, t96584, t96588, t96591, t96633, t96640, t97654, t97680, t97685, t97688, t97699)
}
