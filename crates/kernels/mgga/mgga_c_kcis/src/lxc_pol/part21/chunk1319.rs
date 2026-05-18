//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1319/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1319<F: Float>(t26796: F, t303: F, t4773: F, t1014: F, t27971: F, t27974: F, t7687: F, t26681: F, t26692: F, t26793: F, t26806: F, t27832: F, t27911: F, t4961: F, t8030: F, t93592: F, t95549: F, t96238: F, t96241: F, t96247: F, t96251: F, t96256: F) -> (F, F, F) {
    let t96259 = t303 * t26796 * t4773;
    let t96261 = t1014 * t27971;
    let t96264 = F::new(0.46336805555555555556e-3) * t7687 * t27974;
    let t96265 = F::new(0.46336805555555555556e-3) * t27832 * t26681 + F::new(0.37069444444444444444e-2) * t26692 * t27911 + t96238 - F::new(0.13901041666666666667e-2) * t8030 * t26793 - F::new(0.92673611111111111113e-3) * t93592 * t96241 * t4961 * t26806 - F::new(0.22109259259259259258e-2) * t96247 - F::new(0.73697530864197530861e-3) * t96251 - F::new(0.46336805555555555556e-3) * t93592 * t95549 + F::new(0.11054629629629629629e-2) * t96256 - F::new(0.49745833333333333332e-2) * t96259 + F::new(0.88437037037037037034e-2) * t96261 + t96264;
    (t96259, t96261, t96265)
}
