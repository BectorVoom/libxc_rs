//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1891/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1891(t11195: f64, t11204: f64, t11211: f64, t11213: f64, t14702: f64, t14708: f64, t14713: f64, t14759: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t14868: f64, t14870: f64, t14887: f64, t14890: f64, t14911: f64) -> f64 {
    let t14913 = -t11195 - t11204 + 0.13287407407407407408e0_f64 * t14702 - t14868 + 0.29896666666666666667e0_f64 * t14708 - t14870 + 0.82156666666666666667e-1_f64 * t14713 + 0.1898925e1_f64 * t14759 + 0.18257037037037037037e0_f64 * t11211 + 0.18257037037037037037e-1_f64 * t11213 + t14887 + 0.36514074074074074075e-1_f64 * t14779 - t14890 - 0.54771111111111111112e-1_f64 * t14784 - 0.27385555555555555556e-1_f64 * t14787 - 0.16431333333333333333e0_f64 * t14790 + 0.32862666666666666666e0_f64 * t14793 + 0.16431333333333333333e0_f64 * t14796 + 0.49293999999999999999e0_f64 * t14799 + 0.142419375e1_f64 * t14802 - 0.76790625e-1_f64 * t14805 + t14911;
    t14913
}
