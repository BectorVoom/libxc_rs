//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta914 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta914(t15421: f64, t19318: f64, t15101: f64, t19321: f64, t11299: f64, t23565: f64, t934: f64, t2924: f64, t4631: f64, t6110: f64, t11404: f64, t11450: f64, t11548: f64, t15104: f64, t15350: f64, t15406: f64, t1621: f64, t1622: f64, t19226: f64, t19272: f64, t19275: f64, t19276: f64, t19290: f64, t23723: f64, t23758: f64, t23773: f64, t2943: f64, t2968: f64, t4669: f64, t4670: f64, t6158: f64, t6173: f64, t63971: f64, t953: f64, t4595: f64, t63677: f64, t4636: f64, t64336: f64, t19327: f64, t19331: f64, t19324: f64, t52508: f64, t19250: f64, t19256: f64, t52224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78246, t78248, t78251, t78254, t78279) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947(t15421, t19318, t15101, t19321, t11299, t23565, t934, t2924, t4631, t6110, t11404, t11450, t11548, t15104, t15350, t15406, t1621, t1622, t19226, t19272, t19275, t19276, t19290, t23723, t23758, t23773, t2943, t2968, t4669, t4670, t6158, t6173, t63971, t953);
        let (t78303, t78305, t78307, t78309, t78311, t78313, t78315) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948(t4595, t63677, t4636, t64336, t15101, t19327, t15421, t19331, t19324, t52508, t19250, t19256, t52224);
    (t78246, t78248, t78251, t78254, t78279, t78303, t78305, t78307, t78309, t78311, t78313, t78315)
}
