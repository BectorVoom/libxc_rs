//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 962/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk962(t1371: f64, t2291: f64, t1359: f64, t2252: f64, t1351: f64, t2250: f64, t1363: f64, t2289: f64, t2183: f64, t3353: f64, t8651: f64, t6530: f64, t6533: f64, t6614: f64, t6616: f64, t6619: f64, t6622: f64, t8648: f64, t8654: f64, t8656: f64, t8659: f64, t8661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8815 = t1371 * t2291;
    let t8818 = t1359 * t2252;
    let t8821 = t1351 * t2250;
    let t8824 = t1363 * t2289;
    let t8828 = 2.0_f64 * t2183 * t3353;
    let t8830 = 0.59793333333333333334e0_f64 * t8651;
    let t8840 = 0.8969e0_f64 * t8648 - t8830 - 0.1898925e1_f64 * t8654 - 0.9494625e0_f64 * t8656 + 0.3071625e0_f64 * t8659 + 0.15358125e0_f64 * t8661 + 0.79724444444444444446e0_f64 * t6530 - 0.29896666666666666667e0_f64 * t6533 - t6614 + 0.54771111111111111111e0_f64 * t6616 - 0.16431333333333333333e0_f64 * t6619 - 0.16431333333333333333e0_f64 * t6622;
    (t8815, t8818, t8821, t8824, t8828, t8830, t8840)
}
