//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 440/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk440<F: Float>(t222: F, t233: F, t3462: F, t1297: F, t560: F, t1152: F, t1157: F, t1625: F, t3283: F, t295: F, t559: F, t294: F, t1156: F, t1624: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t3463 = t233 * t3462;
    let t3464 = F::new(1.0) / t1297;
    let t3465 = sigma0 * t3464;
    let t3466 = t3465 * t560;
    let t3468 = t1152 * t1157;
    let t3470 = t1152 * t1625;
    let t3472 = piecewise3::<f64>(t223, F::new(0.0), t3283);
    let t3473 = t295 * t3472;
    let t3474 = t3473 * t559;
    let t3475 = t294 * t3474;
    let t3477 = t1156 * t1624;
    (t3463, t3465, t3466, t3468, t3470, t3473, t3474, t3475, t3477)
}
