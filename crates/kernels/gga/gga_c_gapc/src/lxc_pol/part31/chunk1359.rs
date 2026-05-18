//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1359/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1359<F: Float>(t35458: F, t35463: F, t35466: F, t35471: F, t35475: F, t35478: F, t35480: F, t35485: F, t35489: F, t35493: F, t35495: F, t35500: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36358 = F::new(0.14762395597096631476e-5) * t35458;
    let t36361 = F::new(0.36207601172307334926e-6) * t35463;
    let t36362 = F::new(0.36207601172307334926e-6) * t35466;
    let t36363 = F::new(0.79204127564422295151e-7) * t35471;
    let t36364 = F::new(0.47522476538653377092e-5) * t35475;
    let t36365 = F::new(0.47522476538653377092e-5) * t35478;
    let t36366 = F::new(0.5061392776147416506e-5) * t35480;
    let t36368 = F::new(0.45552534985326748556e-4) * t35485;
    let t36369 = F::new(0.5061392776147416506e-5) * t35489;
    let t36370 = F::new(0.5061392776147416506e-5) * t35493;
    let t36371 = F::new(0.2530696388073708253e-5) * t35495;
    let t36372 = F::new(0.86898242813537603825e-4) * t35500;
    (t36358, t36361, t36362, t36363, t36364, t36365, t36366, t36368, t36369, t36370, t36371, t36372)
}
