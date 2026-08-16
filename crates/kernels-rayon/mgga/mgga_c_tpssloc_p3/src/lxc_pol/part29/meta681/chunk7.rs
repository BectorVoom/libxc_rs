//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2300/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2300(t24826: f64, t27540: f64, t1235: f64, t14706: f64, t24812: f64, t24813: f64, t27478: f64, t27489: f64, t27491: f64, t27724: f64, t3477: f64, t3502: f64, t3604: f64, t3610: f64, t4978: f64, t5068: f64, t7283: f64, t7362: f64, t7363: f64, t8077: f64, t85941: f64, t85943: f64, t85945: f64, t85952: f64, t85955: f64, t94986: f64) -> f64 {
    let t95069 = 0.54831135561607547884e-2_f64 * t24826 * t27540;
    let t95087 = 0.3289868133696452873e-1_f64 * t24812 * t24813 * t3502 * t1235 * t27491 + 0.3289868133696452873e-1_f64 * t24812 * t27489 * t94986 * t4978 - t95069 + 4.0_f64 * t3610 * t27724 * t5068 - 0.36554090374405031922e-2_f64 * t85941 - 0.91385225936012579807e-3_f64 * t85943 - 0.18277045187202515961e-2_f64 * t85945 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t7363 * t14706 + 0.12184696791468343974e-2_f64 * t85952 - 0.82246703342411321825e-2_f64 * t7283 * t3477 * t8077 + 0.27415567780803773942e-2_f64 * t85955 + 2.0_f64 * t3604 * t27478;
    t95087
}
