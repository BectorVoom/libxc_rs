//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2313;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta625<F: Float>(t225: F, t24864: F, t494: F, t1210: F, t1274: F, t1775: F, t17995: F, t18059: F, t1829: F, t20697: F, t20700: F, t20753: F, t21394: F, t21621: F, t24509: F, t24515: F, t24519: F, t24525: F, t24698: F, t460: F, t495: F, t5220: F, t5417: F, t6574: F, t6580: F, t6745: F, t1211: F, t24713: F, t1828: F, t6587: F, t1277: F, t6573: F, t24543: F, t487: F, t13143: F, t489: F, t1287: F, t1794: F, t6695: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24866, t24881) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2313::<F>(t225, t24864, t494, t1210, t1274, t1775, t17995, t18059, t1829, t20697, t20700, t20753, t21394, t21621, t24509, t24515, t24519, t24525, t24698, t460, t495, t5220, t5417, t6574, t6580, t6745);
        let (t24892, t24900, t24906, t24911, t24912, t24915, t24919) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2314::<F>(t1211, t24713, t1828, t6587, t1277, t6573, t24543, t487, t13143, t24864, t489, t1287, t1794, t6695);
    (t24866, t24881, t24892, t24900, t24906, t24911, t24912, t24915, t24919)
}
