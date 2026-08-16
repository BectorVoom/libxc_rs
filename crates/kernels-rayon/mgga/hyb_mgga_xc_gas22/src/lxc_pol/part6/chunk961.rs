//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 961/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk961(t2251: f64, t2273: f64, t271: f64, t3371: f64, t3390: f64, t6678: f64, t6683: f64, t6710: f64, t6722: f64, t8623: f64, t8627: f64, t8725: f64, t8733: f64, t8785: f64, t8788: f64, t8791: f64, t8795: f64, t8798: f64, t8802: f64, t8810: f64) -> f64 {
    let t8813 = -t8623 - t8627 - 4.0_f64 * t6722 * t3371 + 0.64327917994770140268e2_f64 * t6678 * t3390 - 4.0_f64 * t2251 * t8785 - 2.0_f64 * t2251 * t8788 - 0.19298375398431042081e3_f64 * t6683 * t8791 + 0.64327917994770140268e2_f64 * t2273 * t8795 + 0.32163958997385070134e2_f64 * t2273 * t8798 + 0.2069040516770936012e4_f64 * t6710 * t8802 - 0.310907e-1_f64 * t8810 * t271 + t8725 - t8733;
    t8813
}
