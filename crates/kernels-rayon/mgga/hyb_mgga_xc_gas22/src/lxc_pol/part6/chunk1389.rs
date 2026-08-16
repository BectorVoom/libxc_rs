//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1389/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1389(t29851: f64, t29853: f64, t29855: f64, t29857: f64, t29860: f64, t29862: f64, t29865: f64, t29867: f64, t29870: f64, t29873: f64, t29877: f64, t29880: f64) -> f64 {
    let t30164 = -0.1898925e1_f64 * t29851 - 0.9494625e0_f64 * t29853 - 0.76790625e-1_f64 * t29855 + 0.3071625e0_f64 * t29857 + 0.3071625e0_f64 * t29860 + 0.15358125e0_f64 * t29862 + 0.5696775e1_f64 * t29865 - 0.3071625e0_f64 * t29867 + 0.27385555555555555555e0_f64 * t29870 - 0.65725333333333333333e0_f64 * t29873 + 0.49294e0_f64 * t29877 - 0.32862666666666666666e0_f64 * t29880;
    t30164
}
