//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2432/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432(t1041: f64, t13969: f64, t14188: f64, t1046: f64, t10898: f64, t10949: f64, t13977: f64, t13982: f64, t13987: f64, t1618: f64, t3043: f64, t42595: f64, t43120: f64, t43322: f64, t43343: f64, t4596: f64, t4652: f64, t49721: f64, t49732: f64, t49734: f64, t49740: f64, t49743: f64) -> f64 {
    let t49748 = t1041 * t13969 * t14188;
    let t49750 = t49721 / 1536.0_f64 + t43343 * t4596 / 512.0_f64 + t10949 * t13977 / 256.0_f64 + t10949 * t13982 / 512.0_f64 + 3.0_f64 / 512.0_f64 * t43322 * t13987 + t49732 / 48.0_f64 + t49734 / 1536.0_f64 - t10898 * t4652 / 96.0_f64 - t43120 * t1618 / 192.0_f64 - t49740 * t1046 / 144.0_f64 + t49743 * t3043 / 192.0_f64 + 5.0_f64 / 7776.0_f64 * t42595 + 5.0_f64 / 2592.0_f64 * t49748;
    t49750
}
