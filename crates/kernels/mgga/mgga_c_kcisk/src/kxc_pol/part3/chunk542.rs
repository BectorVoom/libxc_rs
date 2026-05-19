//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 542/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk542<F: Float>(t4224: F, t4227: F, t4233: F, t4238: F, t4242: F, t4298: F, t4302: F, t4307: F, t4310: F, t4314: F, t4316: F, t4318: F) -> F {
    let t4564 = F::new(0.20234375e-1) * t4224 - F::cast_from(0.10791666666666666667e0_f64) * t4227 + F::cast_from(0.26979166666666666666e-1_f64) * t4233 - F::new(0.20234375e-1) * t4238 - F::cast_from(0.20833333333333333333e-1_f64) * t4242 + F::new(0.9375e-1) * t4298 - F::cast_from(0.101171875e-1_f64) * t4302 - F::cast_from(0.44965277777777777777e-2_f64) * t4307 - F::cast_from(0.33333333333333333334e0_f64) * t4310 + F::cast_from(0.91666666666666666667e0_f64) * t4314 - F::new(0.5e0) * t4316 + F::new(0.125e0) * t4318;
    t4564
}
