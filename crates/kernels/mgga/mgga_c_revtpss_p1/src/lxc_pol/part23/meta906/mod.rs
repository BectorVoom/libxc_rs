//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta906 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta906<F: Float>(t324: F, t77549: F, t77596: F, t300: F, t1633: F, t52894: F, t64043: F, t972: F, t19331: F, t52514: F, t1610: F, t63610: F, t19056: F, t4632: F, t19327: F, t52645: F, t15416: F, t6142: F, t52505: F, t6110: F, t11450: F, t11461: F, t15241: F, t19272: F, t19276: F, t19304: F, t23711: F, t23714: F, t23785: F, t2982: F, t41788: F, t52440: F, t52511: F, t52637: F, t52837: F, t52840: F, t6173: F, t6190: F, t6209: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t77598, t77600, t77604, t77612, t77622) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912::<F>(t324, t77549, t77596, t300, t1633, t52894, t64043, t972, t19331, t52514, t1610, t63610);
        let (t77624, t77628, t77634, t77636, t77637) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913::<F>(t19056, t4632, t19327, t52645, t15416, t6142, t52505, t6110, t11450, t11461, t15241, t19272, t19276, t19304, t23711, t23714, t23785, t2982, t41788, t52440, t52511, t52637, t52837, t52840, t6173, t6190, t6209, t77612, t77622, t953);
    (t77598, t77600, t77604, t77612, t77622, t77624, t77628, t77634, t77636, t77637)
}
