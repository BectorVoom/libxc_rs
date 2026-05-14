//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 746/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk746<F: Float>(t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F, t6042: F, t6045: F, t6241: F, t1629: F, t1636: F, t187: F, t2128: F, t4475: F, t4480: F, t5896: F, t5898: F, t5899: F, t5902: F, t6049: F, t6220: F, t6222: F, t6225: F, t633: F) -> (F, F) {
    let t6255 = -0.25e0 * t6006 - 0.13489583333333333333e-1 * t6008 - 0.20234375e-1 * t6013 - 0.9375e-1 * t6017 - 0.101171875e-1 * t6021 + 0.625e-1 * t6023 + 0.53958333333333333333e-1 * t6025 + 0.1875e0 * t6030 + 0.625e-1 * t6032 - 0.53958333333333333333e-1 * t6035 - 0.9375e-1 * t6039 - 0.16666666666666666667e0 * t6042 + 0.25e0 * t6045;
    let t6256 = t6241 + t6255;
    let t6260 = t5896 - t5898 - t5899 + t5902 - t6049 + t187 * (-t1629 * t6256 - t1636 * t6222 - t2128 * t4475 + 2.0 * t4480 * t6225 + t6220 * t633 - t5896 + t5898 + t5899 - t5902 + t6049);
    (t6256, t6260)
}
