//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 732/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk732<F: Float>(t406: F, t8749: F, t8697: F, t2972: F, t398: F, t393: F, t8639: F, t8642: F, t1065: F, t2975: F, t401: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8765 = t406 * t8749;
    let t8772 = t406 * t8697;
    let t8785 = F::new(1.0) / t2972 / t398;
    let t8786 = t393 * t8785;
    let t8831 = F::new(0.16068111111111111111e1) * t8639;
    let t8832 = F::new(0.46308888888888888888e0) * t8642;
    let t8847 = F::new(1.0) / t2972 / t1065;
    let t8848 = t393 * t8847;
    let t8850 = F::new(1.0) / t2975 / t401;
    let t8857 = F::new(0.28842592592592592592e-1) * t8639;
    let t8871 = F::new(0.53272592592592592592e-1) * t8639;
    let t8885 = F::new(0.55403703703703703703e-1) * t8639;
    (t8765, t8772, t8785, t8786, t8831, t8832, t8847, t8848, t8850, t8857, t8871, t8885)
}
