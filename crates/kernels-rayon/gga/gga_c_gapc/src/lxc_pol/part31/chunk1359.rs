//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1359/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1359(t35458: f64, t35463: f64, t35466: f64, t35471: f64, t35475: f64, t35478: f64, t35480: f64, t35485: f64, t35489: f64, t35493: f64, t35495: f64, t35500: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36358 = 0.14762395597096631476e-5_f64 * t35458;
    let t36361 = 0.36207601172307334926e-6_f64 * t35463;
    let t36362 = 0.36207601172307334926e-6_f64 * t35466;
    let t36363 = 0.79204127564422295151e-7_f64 * t35471;
    let t36364 = 0.47522476538653377092e-5_f64 * t35475;
    let t36365 = 0.47522476538653377092e-5_f64 * t35478;
    let t36366 = 0.5061392776147416506e-5_f64 * t35480;
    let t36368 = 0.45552534985326748556e-4_f64 * t35485;
    let t36369 = 0.5061392776147416506e-5_f64 * t35489;
    let t36370 = 0.5061392776147416506e-5_f64 * t35493;
    let t36371 = 0.2530696388073708253e-5_f64 * t35495;
    let t36372 = 0.86898242813537603825e-4_f64 * t35500;
    (t36358, t36361, t36362, t36363, t36364, t36365, t36366, t36368, t36369, t36370, t36371, t36372)
}
