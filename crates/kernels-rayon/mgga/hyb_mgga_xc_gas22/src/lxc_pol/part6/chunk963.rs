//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 963/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk963(t8688: f64, t8691: f64, t6597: f64, t8670: f64, t8673: f64, t8676: f64, t8683: f64, t8685: f64, t8695: f64, t8699: f64, t8703: f64, t8706: f64) -> (f64, f64, f64) {
    let t8846 = 0.32862666666666666666e0_f64 * t8688;
    let t8847 = 0.32862666666666666666e0_f64 * t8691;
    let t8852 = 0.142419375e1_f64 * t8670 - 0.76790625e-1_f64 * t8673 + 0.39862222222222222223e0_f64 * t8676 + 0.1898925e1_f64 * t8683 + 0.3071625e0_f64 * t8685 - t6597 - t8846 - t8847 + 0.24647e0_f64 * t8695 + 0.49294e0_f64 * t8699 + 0.24647e0_f64 * t8703 + 0.27385555555555555555e0_f64 * t8706;
    (t8846, t8847, t8852)
}
