//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1513;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta505<F: Float>(t23263: F, t40864: F, t10697: F, t23114: F, t236: F, t807: F, t23267: F, t2703: F, t23148: F, t854: F, t1559: F, t18599: F, t2661: F, t2662: F, t221: F, t23177: F, t2484: F, t2485: F, t1469: F, t4401: F, t61303: F, t14613: F, t18539: F, t18544: F, t4311: F, t23214: F, t750: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t76835, t76856, t76858, t76878, t76882) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1513::<F>(t23263, t40864, t10697, t23114, t236, t807, t23267, t2703, t23148, t854, t1559, t18599, t2661, t2662);
        let (t76887, t76892, t76947, t76949, t76951) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514::<F>(t221, t23177, t2484, t2485, t1469, t4401, t61303, t14613, t18539, t18544, t4311, t23214, t750);
    (t76835, t76856, t76858, t76878, t76882, t76887, t76892, t76947, t76949, t76951)
}
