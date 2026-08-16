//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta577(t14693: f64, t25270: f64, t14927: f64, t27261: f64, t10778: f64, t1941: f64, t50538: f64, t25222: f64, t4435: f64, t14868: f64, t2661: f64, t93082: f64, t14751: f64, t7045: f64, t14757: f64, t25234: f64, t14738: f64, t7038: f64, t14732: f64, t25245: f64, t14668: f64, t14933: f64, t2482: f64, t25260: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99054, t99056, t99063, t99066, t99069) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926(t14693, t25270, t14927, t27261, t10778, t1941, t50538, t25222, t4435, t14868, t2661, t93082);
        let (t99071, t99073, t99075, t99077, t99081, t99085) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1927(t14751, t7045, t14757, t25234, t14738, t7038, t14732, t25245, t14668, t27261, t14933, t2482, t25260, t814);
    (t99054, t99056, t99063, t99066, t99069, t99071, t99073, t99075, t99077, t99081, t99085)
}
