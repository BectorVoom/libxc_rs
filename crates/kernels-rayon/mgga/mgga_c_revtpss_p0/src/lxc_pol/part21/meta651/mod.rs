//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2437;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta651(t1032: f64, t1040: f64, t11902: f64, t11762: f64, t3241: f64, t11752: f64, t11755: f64, t1011: f64, t3247: f64, t697: f64, t3254: f64, t11789: f64, t11937: f64, t225: f64, t42051: f64, t11783: f64, t3215: f64, t11817: f64, t3211: f64, t1025: f64, t1026: f64, t2434: f64, t371: f64, t11901: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42235, t42240, t42249, t42251, t42254, t42257, t42259) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2437(t1032, t1040, t11902, t11762, t3241, t11752, t11755, t1011, t3247, t697, t3254, t11789, t11937);
        let (t42261, t42268, t42270, t42274, t42277) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2438(t225, t42051, t11783, t3215, t11817, t3211, t1025, t1026, t2434, t371, t11901, t993);
    (t42235, t42240, t42249, t42251, t42254, t42257, t42259, t42261, t42268, t42270, t42274, t42277)
}
