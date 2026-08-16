//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1248;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta367(t1139: f64, t24312: f64, t1132: f64, t1723: f64, t6442: f64, t12327: f64, t12331: f64, t12349: f64, t12352: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64, t24285: f64, t1150: f64, t1131: f64, t12230: f64, t24220: f64, t12227: f64, t1744: f64, t6486: f64, t3479: f64, t16706: f64, t16876: f64, t20276: f64, t20278: f64, t20280: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24265: f64, t24267: f64, t24272: f64, t24275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24313, t24315, t24318, t24320, t24322) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1248(t1139, t24312, t1132, t1723, t6442, t12327, t12331, t12349, t12352, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298);
        let (t24323, t24324, t24326, t24327, t24329, t24330, t24331, t24348) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1249(t24285, t24322, t1150, t1131, t12230, t24220, t12227, t1744, t6486, t3479, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265, t24267, t24272, t24275);
    (t24313, t24315, t24318, t24320, t24323, t24324, t24326, t24327, t24329, t24330, t24331, t24348)
}
