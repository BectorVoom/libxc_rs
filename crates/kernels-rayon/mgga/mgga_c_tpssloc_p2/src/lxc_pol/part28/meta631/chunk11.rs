//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1988/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1988(t87666: f64, t87668: f64, t87679: f64, t13390: f64, t1499: f64, t24251: f64, t24278: f64, t26676: f64, t4166: f64, t4182: f64, t4281: f64, t81980: f64, t81989: f64, t82005: f64, t82011: f64, t82013: f64, t82016: f64, t85003: f64, t87660: f64, t87672: f64, t87676: f64, t92745: f64) -> f64 {
    let t92794 = 0.12793931631041761173e0_f64 * t87666;
    let t92795 = 0.76763589786250567036e-1_f64 * t87668;
    let t92798 = 0.3289868133696452873e-1_f64 * t87679;
    let t92803 = t85003 - t4166 * t24251 + 0.3289868133696452873e-1_f64 * t87660 - 0.23029076935875170111e0_f64 * t81980 + 0.76763589786250567036e-1_f64 * t81989 + 4.0_f64 * t4281 * t92745 * t4182 + 0.76763589786250567036e-1_f64 * t82005 - 2.0_f64 * t13390 * t26676 - t92794 + t92795 - 0.6579736267392905746e-1_f64 * t87672 - 0.3289868133696452873e-1_f64 * t87676 + t92798 + t1499 * t24278 - 0.25587863262083522346e0_f64 * t82011 - 0.76763589786250567036e-1_f64 * t82013 - 0.16449340668482264365e-1_f64 * t82016;
    t92803
}
