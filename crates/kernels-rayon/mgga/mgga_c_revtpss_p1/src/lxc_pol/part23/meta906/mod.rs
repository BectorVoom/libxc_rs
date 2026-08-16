//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta906 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta906(t324: f64, t77549: f64, t77596: f64, t300: f64, t1633: f64, t52894: f64, t64043: f64, t972: f64, t19331: f64, t52514: f64, t1610: f64, t63610: f64, t19056: f64, t4632: f64, t19327: f64, t52645: f64, t15416: f64, t6142: f64, t52505: f64, t6110: f64, t11450: f64, t11461: f64, t15241: f64, t19272: f64, t19276: f64, t19304: f64, t23711: f64, t23714: f64, t23785: f64, t2982: f64, t41788: f64, t52440: f64, t52511: f64, t52637: f64, t52837: f64, t52840: f64, t6173: f64, t6190: f64, t6209: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77598, t77600, t77604, t77612, t77622) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912(t324, t77549, t77596, t300, t1633, t52894, t64043, t972, t19331, t52514, t1610, t63610);
        let (t77624, t77628, t77634, t77636, t77637) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913(t19056, t4632, t19327, t52645, t15416, t6142, t52505, t6110, t11450, t11461, t15241, t19272, t19276, t19304, t23711, t23714, t23785, t2982, t41788, t52440, t52511, t52637, t52837, t52840, t6173, t6190, t6209, t77612, t77622, t953);
    (t77598, t77600, t77604, t77612, t77622, t77624, t77628, t77634, t77636, t77637)
}
