//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1103/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1103<F: Float>(t1356: F, t1668: F, t36992: F, t41932: F, t42196: F, t42201: F, t42205: F, t42207: F, t42217: F, t46589: F, t46867: F, t48000: F, t48009: F, t48011: F, t48014: F, t48017: F, t48022: F, t4965: F, t530: F, t7703: F, t884: F, t8876: F, t9960: F) -> F {
    let t48026 = -F::new(0.25538759935978703639e-4) * t48000 - F::new(0.23948483403727617128e0) * t1356 * t7703 * t46867 - F::new(0.4726e1) * t1668 * t8876 - F::new(0.4726e1) * t530 * t41932 + F::new(0.13637330827122670864e0) * t48009 + F::new(0.27274661654245341728e-1) * t48011 - F::new(0.26668558061928778579e0) * t42196 + F::new(0.44903406381989282115e-1) * t48014 + t36992 - F::new(0.72732431077987577944e-1) * t42201 - F::new(0.5987120850931904282e-1) * t48017 + t42205 - t42207 + F::new(0.59590439850616975157e-4) * t42217 + F::new(0.39914139006212695214e-1) * t4965 * t9960 + F::new(0.79828278012425390427e-1) * t48022 + F::new(0.59871208509319042821e-1) * t884 * t46589;
    t48026
}
