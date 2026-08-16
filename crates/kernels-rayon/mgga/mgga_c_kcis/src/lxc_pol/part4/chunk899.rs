//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 899/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk899(t6192: f64, t6219: f64, t1628: f64, t2118: f64, t1636: f64, t2128: f64, t5906: f64, t5911: f64, t5914: f64, t5917: f64, t5920: f64, t5922: f64, t5924: f64, t5926: f64, t5930: f64, t5933: f64, t5936: f64, t6000: f64, t6003: f64) -> (f64, f64, f64, f64) {
    let t6220 = t6192 + t6219;
    let t6222 = t2118 * t1628;
    let t6225 = t2128 * t1636;
    let t6241 = -0.44965277777777777777e-2_f64 * t5906 - 0.20833333333333333333e-1_f64 * t5911 + 0.13489583333333333333e-1_f64 * t5914 + 0.13489583333333333333e-1_f64 * t5917 - 0.625e-1_f64 * t5920 - 0.13489583333333333333e-1_f64 * t5922 + 0.101171875e-1_f64 * t5924 - 0.9375e-1_f64 * t5926 + 0.101171875e-1_f64 * t5930 - 0.625e-1_f64 * t5933 + 0.71944444444444444444e-1_f64 * t5936 + 0.9375e-1_f64 * t6000 + 0.101171875e-1_f64 * t6003;
    (t6220, t6222, t6225, t6241)
}
