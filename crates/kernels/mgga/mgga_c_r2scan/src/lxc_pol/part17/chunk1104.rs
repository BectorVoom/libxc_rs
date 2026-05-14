//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1104/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1104<F: Float>(t39239: F, t39247: F, t42162: F, t42164: F, t42165: F, t43820: F, t44027: F, t44029: F, t44032: F, t44035: F, t44037: F, t44039: F, t44043: F, t44046: F, t44049: F, t39054: F, t42170: F, t43826: F, t43829: F, t43832: F, t44051: F, t44054: F, t44057: F, t44061: F, t44064: F, t44068: F, t44072: F, t44074: F, t44077: F, t44080: F) -> (F, F) {
    let t44957 = -t44027 + 0.12195059916630011325e-2 * t39239 - t44029 - t44032 + t44035 - t44037 + t44039 + t44043 - t44046 + 0.1921128438866447784e-2 * t43820 + t42162 - 0.1440846329149835838e-2 * t39247 + t42164 + t42165 - t44049;
    let t44962 = t44051 + t44054 + t44057 - 0.72042316457491791901e-3 * t43826 - 0.30487649791575028312e-3 * t43829 + t44061 - t44064 + t44068 - 0.81300399444200075499e-3 * t43832 + t44072 + t44074 - t39054 - t44077 - t44080 - t42170;
    (t44957, t44962)
}
