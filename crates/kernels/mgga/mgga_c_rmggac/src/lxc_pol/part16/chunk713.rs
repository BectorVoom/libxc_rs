//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 713/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk713<F: Float>(t10015: F, t10019: F, t10025: F, t10031: F, t10033: F, t2474: F, t534: F, t72: F, t10041: F, t10046: F, t10051: F, t10054: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10362 = F::new(0.1702583995731913576e-4) * t10015;
    let t10363 = F::new(0.85129199786595678799e-5) * t10019;
    let t10364 = F::new(0.5107751987195740728e-4) * t10025;
    let t10365 = F::new(0.1702583995731913576e-4) * t10031;
    let t10366 = F::new(0.1702583995731913576e-4) * t10033;
    let t10367 = t534 * t2474;
    let t10368 = t72 * t10367;
    let t10369 = F::new(2.0) * t10368;
    let t10370 = F::new(0.1702583995731913576e-4) * t10041;
    let t10371 = F::new(0.85129199786595678799e-5) * t10046;
    let t10374 = F::new(0.23942587439980034662e-4) * t10051;
    let t10375 = F::new(0.35922725105591425692e0) * t10054;
    (t10362, t10363, t10364, t10365, t10366, t10367, t10369, t10370, t10371, t10374, t10375)
}
