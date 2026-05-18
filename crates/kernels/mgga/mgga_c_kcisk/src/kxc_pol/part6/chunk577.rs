//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 577/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk577<F: Float>(t338: F, t7828: F, t1320: F, t1310: F, t1309: F, t2164: F, t2170: F, t3935: F, t3983: F, t405: F, t6155: F, t6157: F, t6172: F, t6197: F, t8022: F, t8033: F, t8037: F, t8041: F, t8045: F, t8050: F) -> (F, F, F, F) {
    let t400 = F::new(0.0) < t338;
    let t8054 = piecewise3::<f64>(t400, t7828, -t7828);
    let t8055 = t1320 * t8054;
    let t8056 = t1310 * t8055;
    let t8059 = F::new(0.5397236614853195164e-1) * t8022 * t405 + F::new(0.35981577432354634426e-1) * t6155 + F::new(0.35981577432354634426e-1) * t6157 * t2164 - F::new(0.10794473229706390328e0) * t6157 * t2170 - t3983 + F::new(0.11993859144118211475e-1) * t6172 - F::new(0.35981577432354634426e-1) * t6197 + F::new(0.23987718288236422951e-1) * t1309 * t8033 - F::new(0.35981577432354634426e-1) * t3935 * t8037 - F::new(0.35981577432354634426e-1) * t1309 * t8041 + F::new(0.17990788716177317213e-1) * t1309 * t8045 + F::new(0.10794473229706390328e0) * t1309 * t8050 - F::new(0.5397236614853195164e-1) * t1309 * t8056;
    (t8054, t8055, t8056, t8059)
}
