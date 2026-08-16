//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1412/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1412(t35071: f64, t35074: f64, t35080: f64, t35083: f64, t35090: f64, t35093: f64, t35095: f64, t35097: f64, t35077: f64, t35086: f64, t37184: f64, t35108: f64) -> (f64, f64) {
    let t37185 = 0.42206481990611010728e-7_f64 * t35071;
    let t37186 = 0.2698871527777777778e-4_f64 * t35074;
    let t37188 = 0.40483072916666666668e-3_f64 * t35080;
    let t37189 = 0.18310351929594268994e-5_f64 * t35083;
    let t37191 = 0.10298285674687440379e-5_f64 * t35090;
    let t37192 = 0.15716995342493974597e-7_f64 * t35093;
    let t37193 = 0.27012148473991046866e-5_f64 * t35095;
    let t37194 = 0.11594181388521408695e-4_f64 * t35097;
    let t37195 = t37184 - t37185 - t37186 + 0.57970906942607043474e-5_f64 * t35077 - t37188 - t37189 + 0.33460450185846399385e-7_f64 * t35086 + t37191 - t37192 + t37193 + t37194;
    let t37200 = 0.20220636637604418766e-5_f64 * t35108;
    (t37195, t37200)
}
