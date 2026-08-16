//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1988/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1988<F: Float>(t87666: F, t87668: F, t87679: F, t13390: F, t1499: F, t24251: F, t24278: F, t26676: F, t4166: F, t4182: F, t4281: F, t81980: F, t81989: F, t82005: F, t82011: F, t82013: F, t82016: F, t85003: F, t87660: F, t87672: F, t87676: F, t92745: F) -> F {
    let t92794 = F::cast_from(0.12793931631041761173e0_f64) * t87666;
    let t92795 = F::cast_from(0.76763589786250567036e-1_f64) * t87668;
    let t92798 = F::cast_from(0.3289868133696452873e-1_f64) * t87679;
    let t92803 = t85003 - t4166 * t24251 + F::cast_from(0.3289868133696452873e-1_f64) * t87660 - F::cast_from(0.23029076935875170111e0_f64) * t81980 + F::cast_from(0.76763589786250567036e-1_f64) * t81989 + F::cast_from(4.0_f64) * t4281 * t92745 * t4182 + F::cast_from(0.76763589786250567036e-1_f64) * t82005 - F::cast_from(2.0_f64) * t13390 * t26676 - t92794 + t92795 - F::cast_from(0.6579736267392905746e-1_f64) * t87672 - F::cast_from(0.3289868133696452873e-1_f64) * t87676 + t92798 + t1499 * t24278 - F::cast_from(0.25587863262083522346e0_f64) * t82011 - F::cast_from(0.76763589786250567036e-1_f64) * t82013 - F::cast_from(0.16449340668482264365e-1_f64) * t82016;
    t92803
}
