//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1049/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1049<F: Float>(t2292: F, t30221: F, t36402: F, t40679: F, t40716: F, t43677: F, t47202: F, t47207: F, t47213: F, t47215: F, t47219: F, t47223: F, t47225: F, t47229: F, t47233: F, t47235: F, t47238: F, t47242: F, t5928: F, t8933: F) -> F {
    let t47244 = F::new(0.76616279807936110914e-4) * t47202 - F::new(0.10215503974391481455e-3) * t47207 + F::new(0.10000709273223291967e0) * t36402 - F::new(0.82764499792523576607e-4) * t40679 - t43677 + F::new(0.79828278012425390428e-1) * t5928 * t8933 - F::new(0.2993560425465952141e-1) * t47213 + F::new(0.29795219925308487578e-4) * t47215 + F::new(0.79828278012425390428e-1) * t30221 * t2292 + F::new(0.17025839957319135759e-4) * t47219 - F::new(0.85129199786595678796e-5) * t47223 + F::new(0.23942587439980034662e-4) * t47225 - F::new(0.25538759935978703639e-4) * t47229 + F::new(0.25538759935978703639e-4) * t47233 + F::new(0.39914139006212695213e-1) * t47235 - F::new(0.13637330827122670864e-1) * t47238 + F::new(0.36366215538993788971e-1) * t47242 + t40716;
    t47244
}
