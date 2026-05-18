//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 739/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk739<F: Float>(t12884: F, t325: F, t4459: F, t512: F, t507: F, t12998: F, t12974: F, t1527: F, t4462: F, t515: F, t1588: F, t3532: F) -> (F, F, F, F, F, F, F, F) {
    let t14743 = t325 * t12884;
    let t14756 = F::new(1.0) / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14784 = F::new(0.46308888888888888888e0) * t12998;
    let t14785 = F::new(0.16068111111111111111e1) * t12974;
    let t14797 = F::new(1.0) / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = F::new(1.0) / t4462 / t515;
    let t14831 = F::new(0.53272592592592592592e-1) * t12974;
    let t14909 = t1588 * t3532;
    (t14743, t14757, t14784, t14785, t14798, t14800, t14831, t14909)
}
