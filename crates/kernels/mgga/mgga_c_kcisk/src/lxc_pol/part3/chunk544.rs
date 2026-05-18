//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 544/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk544<F: Float>(t227: F, t297: F, t4569: F, t294: F, t3293: F, t565: F, t806: F, t564: F, t1629: F, t2053: F, t1944: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t4570 = t297 * t4569;
    let t4571 = t294 * t4570;
    let t4573 = piecewise3::<f64>(t228, F::new(0.0), t3293);
    let t4574 = t565 * t4573;
    let t4575 = t4574 * t806;
    let t4576 = t564 * t4575;
    let t4578 = t1629 * t2053;
    let t4579 = t564 * t4578;
    let t4581 = t1944 * sigma2;
    (t4570, t4571, t4574, t4576, t4579, t4581)
}
