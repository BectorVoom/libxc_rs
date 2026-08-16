//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta602<F: Float>(t19275: F, t953: F, t4669: F, t4673: F, t11452: F, t6157: F, t6190: F, t972: F, t11409: F, t11450: F, t15104: F, t15350: F, t15406: F, t15413: F, t19258: F, t19263: F, t19266: F, t19269: F, t19272: F, t2943: F, t2968: F, t3012: F, t4652: F, t4674: F, t4690: F, t4712: F) -> (F, F, F, F, F, F) {
        let (t19276, t19279, t19282, t19283, t19290, t19293) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2492::<F>(t19275, t953, t4669, t4673, t11452, t6157, t6190, t972, t11409, t11450, t15104, t15350, t15406, t15413, t19258, t19263, t19266, t19269, t19272, t2943, t2968, t3012, t4652, t4674, t4690, t4712);
    (t19276, t19279, t19282, t19283, t19290, t19293)
}
