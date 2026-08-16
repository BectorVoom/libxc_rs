//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 958/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk958(t14444: f64, t5898: f64, t34813: f64, t5144: f64, t40724: f64, t5267: f64, t235: f64, t26087: f64, t5888: f64, t15516: f64, t4965: f64, t74984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77335 = t14444 * t5898;
    let t77337 = 0.35922725105591425692e0_f64 * t34813 * t77335;
    let t77338 = t14444 * t5144;
    let t77340 = 0.35922725105591425692e0_f64 * t40724 * t77338;
    let t77341 = t14444 * t5267;
    let t77343 = 0.35922725105591425692e0_f64 * t34813 * t77341;
    let t77344 = t235 * t26087;
    let t77345 = t14444 * t5888;
    let t77347 = 0.47896966807455234256e0_f64 * t77344 * t77345;
    let t77349 = 0.39914139006212695214e-1_f64 * t4965 * t15516;
    let t77352 = 0.40911992481368012592e-1_f64 * t74984;
    (t77335, t77337, t77338, t77340, t77341, t77343, t77345, t77347, t77349, t77352)
}
