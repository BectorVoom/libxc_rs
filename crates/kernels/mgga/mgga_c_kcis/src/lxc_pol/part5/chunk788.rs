//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 788/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk788<F: Float>(t6192: F, t6219: F, t1628: F, t2118: F, t1636: F, t2128: F, t5906: F, t5911: F, t5914: F, t5917: F, t5920: F, t5922: F, t5924: F, t5926: F, t5930: F, t5933: F, t5936: F, t6000: F, t6003: F) -> (F, F, F, F) {
    let t6220 = t6192 + t6219;
    let t6222 = t2118 * t1628;
    let t6225 = t2128 * t1636;
    let t6241 = -F::cast_from(0.44965277777777777777e-2_f64) * t5906 - F::cast_from(0.20833333333333333333e-1_f64) * t5911 + F::cast_from(0.13489583333333333333e-1_f64) * t5914 + F::cast_from(0.13489583333333333333e-1_f64) * t5917 - F::new(0.625e-1) * t5920 - F::cast_from(0.13489583333333333333e-1_f64) * t5922 + F::cast_from(0.101171875e-1_f64) * t5924 - F::new(0.9375e-1) * t5926 + F::cast_from(0.101171875e-1_f64) * t5930 - F::new(0.625e-1) * t5933 + F::cast_from(0.71944444444444444444e-1_f64) * t5936 + F::new(0.9375e-1) * t6000 + F::cast_from(0.101171875e-1_f64) * t6003;
    (t6220, t6222, t6225, t6241)
}
