//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1231;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta321(t1284: f64, t3566: f64, t3624: f64, t1250: f64, t12718: f64, t3720: f64, t126: f64, t482: f64, t828: f64, t3722: f64, t3718: f64, t1214: f64, t2251: f64, t5268: f64, t1042: f64, t11231: f64, t1261: f64, t12847: f64, t12853: f64, t12855: f64, t12858: f64, t12862: f64, t12866: f64, t12868: f64, t12872: f64, t12876: f64, t12882: f64, t12887: f64, t12890: f64, t12893: f64, t12895: f64, t12900: f64, t12902: f64, t12905: f64, t12907: f64, t3711: f64, t484: f64, t5331: f64, t5340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12909, t12910, t12911, t12912, t12915, t12916) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1231(t1284, t3566, t3624, t1250, t12718, t3720, t126, t482, t828);
        let (t12917, t12921, t12922, t12925, t12926, t12929) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1232(t12916, t3722, t3718, t1214, t2251, t5268, t1042, t11231, t1261, t12847, t12853, t12855, t12858, t12862, t12866, t12868, t12872, t12876, t12882, t12887, t12890, t12893, t12895, t12900, t12902, t12905, t12907, t12910, t12912, t3711, t484, t5331, t5340);
    (t12909, t12910, t12911, t12912, t12915, t12916, t12917, t12921, t12922, t12925, t12926, t12929)
}
