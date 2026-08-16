//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1029/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1029(t10980: f64, t10983: f64, t10986: f64, t11002: f64, t11010: f64, t11015: f64, t11020: f64, t11053: f64, t11056: f64, t11059: f64, t11062: f64, t11065: f64, t11068: f64, t11071: f64, t11098: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8627: f64, t8629: f64, t8631: f64) -> f64 {
    let t11100 = 0.99655555555555555557e-1_f64 * t8605 + 0.66437037037037037038e-1_f64 * t8607 - 0.26574814814814814816e0_f64 * t8616 - 0.19931111111111111111e0_f64 * t8618 - 0.18257037037037037037e0_f64 * t8627 + 0.54771111111111111111e-1_f64 * t8629 + 0.18257037037037037037e-1_f64 * t8631 - 0.13287407407407407408e0_f64 * t10980 + t10983 - 0.29896666666666666667e0_f64 * t10986 + t11053 - 0.54771111111111111112e-1_f64 * t11056 - 0.27385555555555555556e-1_f64 * t11059 - 0.36514074074074074075e-1_f64 * t11062 + 0.32862666666666666666e0_f64 * t11065 + 0.16431333333333333333e0_f64 * t11068 + 0.13287407407407407408e0_f64 * t11002 - t11071 - 0.33218518518518518518e0_f64 * t11010 + 0.11958666666666666667e1_f64 * t11015 - 0.39862222222222222222e0_f64 * t11020 + t11098;
    t11100
}
