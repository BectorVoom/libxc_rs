//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 478/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk478<F: Float>(t3571: F, t1527: F, t512: F, t507: F, t3657: F, t515: F, t325: F, t3696: F, t3722: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4423 = F::new(0.22831111111111111111e-1) * t3571;
    let t4434 = t1527 * t512;
    let t4435 = F::new(1.0) / t4434;
    let t4436 = t507 * t4435;
    let t4443 = F::new(0.68863333333333333333e0) * t3571;
    let t4450 = F::new(0.17365833333333333333e0) * t3657;
    let t4459 = t1527 * t1527;
    let t4460 = F::new(1.0) / t4459;
    let t4461 = t507 * t4460;
    let t4462 = t515 * t515;
    let t4463 = F::new(1.0) / t4462;
    let t4471 = t325 * t3696;
    let t4478 = t325 * t3722;
    (t4423, t4435, t4436, t4443, t4450, t4459, t4460, t4461, t4462, t4463, t4471, t4478)
}
