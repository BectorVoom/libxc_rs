//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 605/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk605<F: Float>(t1636: F, t2128: F, t5906: F, t5911: F, t5914: F, t5917: F, t5920: F, t5922: F, t5924: F, t5926: F, t5930: F, t5933: F, t5936: F, t6000: F, t6003: F) -> (F, F) {
    let t6225 = t2128 * t1636;
    let t6241 = -F::new(0.44965277777777777777e-2) * t5906 - F::new(0.20833333333333333333e-1) * t5911 + F::new(0.13489583333333333333e-1) * t5914 + F::new(0.13489583333333333333e-1) * t5917 - F::new(0.625e-1) * t5920 - F::new(0.13489583333333333333e-1) * t5922 + F::new(0.101171875e-1) * t5924 - F::new(0.9375e-1) * t5926 + F::new(0.101171875e-1) * t5930 - F::new(0.625e-1) * t5933 + F::new(0.71944444444444444444e-1) * t5936 + F::new(0.9375e-1) * t6000 + F::new(0.101171875e-1) * t6003;
    (t6225, t6241)
}
