//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 616/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk616<F: Float>(t1636: F, t2128: F, t5906: F, t5911: F, t5914: F, t5917: F, t5920: F, t5922: F, t5924: F, t5926: F, t5930: F, t5933: F, t5936: F, t6000: F, t6003: F, t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F, t6042: F, t6045: F) -> (F, F, F) {
    let t6225 = t2128 * t1636;
    let t6241 = -0.44965277777777777777e-2 * t5906 - 0.20833333333333333333e-1 * t5911 + 0.13489583333333333333e-1 * t5914 + 0.13489583333333333333e-1 * t5917 - 0.625e-1 * t5920 - 0.13489583333333333333e-1 * t5922 + 0.101171875e-1 * t5924 - 0.9375e-1 * t5926 + 0.101171875e-1 * t5930 - 0.625e-1 * t5933 + 0.71944444444444444444e-1 * t5936 + 0.9375e-1 * t6000 + 0.101171875e-1 * t6003;
    let t6255 = -0.25e0 * t6006 - 0.13489583333333333333e-1 * t6008 - 0.20234375e-1 * t6013 - 0.9375e-1 * t6017 - 0.101171875e-1 * t6021 + 0.625e-1 * t6023 + 0.53958333333333333333e-1 * t6025 + 0.1875e0 * t6030 + 0.625e-1 * t6032 - 0.53958333333333333333e-1 * t6035 - 0.9375e-1 * t6039 - 0.16666666666666666667e0 * t6042 + 0.25e0 * t6045;
    (t6225, t6241, t6255)
}
