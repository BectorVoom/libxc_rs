//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta686 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta686(t12772: f64, t12780: f64, t3625: f64, t13052: f64, t13054: f64, t3172: f64, t11262: f64, t3711: f64, t3713: f64, t12657: f64, t1284: f64, t3624: f64, t12875: f64, t12916: f64, t5331: f64, t12871: f64, t5340: f64, t1222: f64, t12282: f64, t17471: f64, t1261: f64, t12944: f64, t12932: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44729, t44748, t44751, t44769) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503(t12772, t12780, t3625, t13052, t13054, t3172, t11262, t3711, t3713, t12657, t1284, t3624);
        let (t44773, t44776, t44786, t44789, t44792) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504(t12875, t12916, t5331, t12871, t5340, t1222, t12282, t17471, t1261, t12944, t3172, t12932, t3711);
    (t44729, t44748, t44751, t44769, t44773, t44776, t44786, t44789, t44792)
}
