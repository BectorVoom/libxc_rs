//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta914 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta914<F: Float>(t15421: F, t19318: F, t15101: F, t19321: F, t11299: F, t23565: F, t934: F, t2924: F, t4631: F, t6110: F, t11404: F, t11450: F, t11548: F, t15104: F, t15350: F, t15406: F, t1621: F, t1622: F, t19226: F, t19272: F, t19275: F, t19276: F, t19290: F, t23723: F, t23758: F, t23773: F, t2943: F, t2968: F, t4669: F, t4670: F, t6158: F, t6173: F, t63971: F, t953: F, t4595: F, t63677: F, t4636: F, t64336: F, t19327: F, t19331: F, t19324: F, t52508: F, t19250: F, t19256: F, t52224: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t78246, t78248, t78251, t78254, t78279) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947::<F>(t15421, t19318, t15101, t19321, t11299, t23565, t934, t2924, t4631, t6110, t11404, t11450, t11548, t15104, t15350, t15406, t1621, t1622, t19226, t19272, t19275, t19276, t19290, t23723, t23758, t23773, t2943, t2968, t4669, t4670, t6158, t6173, t63971, t953);
        let (t78303, t78305, t78307, t78309, t78311, t78313, t78315) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948::<F>(t4595, t63677, t4636, t64336, t15101, t19327, t15421, t19331, t19324, t52508, t19250, t19256, t52224);
    (t78246, t78248, t78251, t78254, t78279, t78303, t78305, t78307, t78309, t78311, t78313, t78315)
}
