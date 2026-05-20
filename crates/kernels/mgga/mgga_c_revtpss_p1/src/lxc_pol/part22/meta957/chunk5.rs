//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3212/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3212<F: Float>(t10355: F, t10368: F, t13312: F, t13325: F, t13328: F, t1480: F, t18281: F, t21732: F, t21733: F, t21736: F, t21741: F, t21742: F, t21761: F, t2251: F, t2258: F, t2275: F, t2282: F, t4201: F, t4210: F, t44: F, t46065: F, t46074: F, t56: F, t5819: F, t5825: F, t606: F, t614: F) -> F {
    let t60987 = F::new(5.0) / F::new(162.0) * t56 * t46074 * t5819 * t2251 + F::new(5.0) / F::new(9.0) * t56 * t4210 * t13312 + F::new(5.0) / F::new(9.0) * t56 * t2282 * t18281 * t606 + F::new(5.0) / F::new(18.0) * t56 * t21761 * t2258 + F::new(5.0) / F::new(108.0) * t56 * t10368 * t5825 * t2251 - F::new(80.0) / F::new(27.0) * t614 * t21736 + F::new(20.0) / F::new(81.0) * t614 * t21733 - F::new(5.0) / F::new(108.0) * t44 * t21732 * t2258 + F::new(5.0) / F::new(162.0) * t44 * t46065 * t5819 * t2251 + F::new(5.0) / F::new(9.0) * t44 * t4201 * t13312 - F::new(40.0) / F::new(27.0) * t614 * t21742 + F::new(5.0) / F::new(9.0) * t44 * t2275 * t18281 * t606 + F::new(5.0) / F::new(18.0) * t44 * t21741 * t2258 - F::new(5.0) / F::new(108.0) * t44 * t10355 * t5825 * t2251 - F::new(80.0) / F::new(27.0) * t1480 * t13325 - F::new(40.0) / F::new(27.0) * t1480 * t13328;
    t60987
}
