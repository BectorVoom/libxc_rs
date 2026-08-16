//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta329(t13044: f64, t13063: f64, t1042: f64, t1032: f64, t3552: f64, t1246: f64, t1250: f64, t12732: f64, t482: f64, t1263: f64, t3568: f64, t1122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13064, t13065, t13069, t13075, t13076, t13079, t13080) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1245(t13044, t13063, t1042, t1032, t3552, t1246, t1250, t12732, t482, t1263, t3568, t1122);
    (t13064, t13065, t13069, t13075, t13076, t13079, t13080)
}
