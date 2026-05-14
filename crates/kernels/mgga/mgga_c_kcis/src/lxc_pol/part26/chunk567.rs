//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 567/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk567<F: Float>(t6027: F, t6029: F, t1529: F, t2047: F, t1547: F, t2061: F, t1546: F, t556: F, t5627: F, t572: F, t1533: F, t1538: F, t2042: F, t571: F, t1534: F, t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6030 = t6027 * t6029;
    let t6032 = t1529 * t2047;
    let t6034 = t2061 * t1547;
    let t6035 = t1546 * t6034;
    let t6037 = t556 * t5627;
    let t6038 = t572 * t6037;
    let t6039 = t1533 * t6038;
    let t6041 = t2042 * t1538;
    let t6042 = t571 * t6041;
    let t6044 = t2042 * t1534;
    let t6045 = t1533 * t6044;
    let t6047 = -t6006 / 6.0 - t6008 / 192.0 - t6013 / 128.0 - t6017 / 16.0 - t6021 / 256.0 + t6023 / 24.0 + t6025 / 48.0 + t6030 / 8.0 + t6032 / 24.0 - t6035 / 48.0 - t6039 / 16.0 - t6042 / 9.0 + t6045 / 6.0;
    (t6030, t6032, t6034, t6035, t6037, t6038, t6039, t6041, t6042, t6044, t6045, t6047)
}
