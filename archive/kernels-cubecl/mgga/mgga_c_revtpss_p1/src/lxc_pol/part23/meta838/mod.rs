//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta838 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2710;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta838<F: Float>(t17373: F, t21203: F, t1230: F, t21271: F, t1263: F, t21082: F, t17544: F, t5293: F, t21275: F, t17769: F, t5381: F, t5391: F, t1247: F, t20902: F, t3172: F, t1234: F, t17209: F, t17505: F, t12855: F, t12916: F, t21120: F, t21093: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69721, t69723, t69742, t69773, t69783, t69787, t69789) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2710::<F>(t17373, t21203, t1230, t21271, t1263, t21082, t17544, t5293, t21275, t17769, t5381, t5391);
        let (t69793, t69795, t69812, t69820, t69832) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2711::<F>(t1247, t20902, t3172, t1234, t21271, t17209, t17505, t12855, t12916, t21120, t21093, t372);
    (t69721, t69723, t69742, t69773, t69783, t69787, t69789, t69793, t69795, t69812, t69820, t69832)
}
