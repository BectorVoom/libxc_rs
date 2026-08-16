//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2313;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta625(t225: f64, t24864: f64, t494: f64, t1210: f64, t1274: f64, t1775: f64, t17995: f64, t18059: f64, t1829: f64, t20697: f64, t20700: f64, t20753: f64, t21394: f64, t21621: f64, t24509: f64, t24515: f64, t24519: f64, t24525: f64, t24698: f64, t460: f64, t495: f64, t5220: f64, t5417: f64, t6574: f64, t6580: f64, t6745: f64, t1211: f64, t24713: f64, t1828: f64, t6587: f64, t1277: f64, t6573: f64, t24543: f64, t487: f64, t13143: f64, t489: f64, t1287: f64, t1794: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24866, t24881) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2313(t225, t24864, t494, t1210, t1274, t1775, t17995, t18059, t1829, t20697, t20700, t20753, t21394, t21621, t24509, t24515, t24519, t24525, t24698, t460, t495, t5220, t5417, t6574, t6580, t6745);
        let (t24892, t24900, t24906, t24911, t24912, t24915, t24919) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2314(t1211, t24713, t1828, t6587, t1277, t6573, t24543, t487, t13143, t24864, t489, t1287, t1794, t6695);
    (t24866, t24881, t24892, t24900, t24906, t24911, t24912, t24915, t24919)
}
