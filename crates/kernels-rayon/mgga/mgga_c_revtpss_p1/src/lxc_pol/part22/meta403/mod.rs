//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1996;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta403(t14066: f64, t225: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t213: f64, t4071: f64, t561: f64, t5728: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64, t9691: f64, t9694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14067, t14078, t14079, t14081, t14082, t14084, t14085, t14087) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1996(t14066, t225, t5774, t72, t686, t3915, t5711, t786, t1364, t1357, t5775, t689);
        let t14088 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1997(t14067, t14081, t14084, t14087, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
    (t14067, t14078, t14079, t14081, t14082, t14084, t14085, t14087, t14088)
}
