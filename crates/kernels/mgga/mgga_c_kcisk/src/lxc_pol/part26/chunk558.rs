//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 558/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk558<F: Float>(t1592: F, t4419: F, t535: F, t3571: F, t1524: F, t1528: F, t1527: F, t512: F, t507: F, t3657: F, t515: F, t1197: F, t1203: F, t325: F, t3696: F, t3722: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4420 = t4419 * t1592;
    let t4421 = t535 * t4420;
    let t4423 = 0.22831111111111111111e-1 * t3571;
    let t4431 = t1524 * t1528;
    let t4434 = t1527 * t512;
    let t4435 = 1.0 / t4434;
    let t4436 = t507 * t4435;
    let t4443 = 0.68863333333333333333e0 * t3571;
    let t4450 = 0.17365833333333333333e0 * t3657;
    let t4459 = t1527 * t1527;
    let t4460 = 1.0 / t4459;
    let t4461 = t507 * t4460;
    let t4462 = t515 * t515;
    let t4463 = 1.0 / t4462;
    let t4468 = t1197 * t1203;
    let t4471 = t325 * t3696;
    let t4478 = t325 * t3722;
    (t4420, t4421, t4423, t4431, t4435, t4436, t4443, t4450, t4459, t4460, t4461, t4462, t4463, t4468, t4471, t4478)
}
