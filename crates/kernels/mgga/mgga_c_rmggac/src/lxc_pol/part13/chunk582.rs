//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 582/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk582<F: Float>(t739: F, t8078: F, t7405: F, t7412: F, t7422: F, t7425: F, t7436: F, t7440: F, t7442: F, t7445: F, t7451: F, t7458: F, t7464: F, t7470: F, t7479: F, t7485: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8079 = t739 * t8078;
    let t8080 = 0.59871208509319042821e-1 * t8079;
    let t8087 = 0.1702583995731913576e-4 * t7405;
    let t8088 = 0.23942587439980034662e-4 * t7412;
    let t8090 = 0.1702583995731913576e-4 * t7422;
    let t8091 = 0.5107751987195740728e-4 * t7425;
    let t8093 = 0.85129199786595678799e-5 * t7436;
    let t8095 = 0.11974241701863808564e0 * t7440;
    let t8096 = 0.5987120850931904282e-1 * t7442;
    let t8097 = 0.8980681276397856423e-1 * t7445;
    let t8098 = 0.1702583995731913576e-4 * t7451;
    let t8099 = 0.212822999466489197e-4 * t7458;
    let t8100 = 0.1702583995731913576e-4 * t7464;
    let t8101 = 0.5107751987195740728e-4 * t7470;
    let t8102 = 0.1702583995731913576e-4 * t7479;
    let t8103 = 0.5107751987195740728e-4 * t7485;
    (t8080, t8087, t8088, t8090, t8091, t8093, t8095, t8096, t8097, t8098, t8099, t8100, t8101, t8102, t8103)
}
