//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 658/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk658<F: Float>(t9813: F, t9815: F, t9818: F, t9820: F, t530: F, t9343: F, t2211: F, t6557: F, t884: F, t1763: F, t8041: F, t1356: F, t9827: F, t9832: F, t9836: F, t2466: F, t4985: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10299 = 0.5987120850931904282e-1 * t9813;
    let t10301 = 0.5107751987195740728e-4 * t9815;
    let t10302 = 0.2553875993597870364e-4 * t9818;
    let t10303 = 0.1702583995731913576e-4 * t9820;
    let t10304 = t530 * t9343;
    let t10305 = 0.4726e1 * t10304;
    let t10306 = t2211 * t6557;
    let t10307 = t884 * t10306;
    let t10308 = 0.23948483403727617128e0 * t10307;
    let t10309 = t8041 * t1763;
    let t10310 = t1356 * t10309;
    let t10311 = 0.11974241701863808564e0 * t10310;
    let t10312 = 0.85129199786595678799e-5 * t9827;
    let t10313 = 0.13637330827122670865e0 * t9832;
    let t10314 = 0.13637330827122670865e-1 * t9836;
    let t10315 = t4985 * t2466;
    (t10299, t10301, t10302, t10303, t10305, t10306, t10308, t10309, t10311, t10312, t10313, t10314, t10315)
}
