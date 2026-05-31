//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1208/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1208<F: Float>(t3275: F, t3582: F, t42331: F, t44014: F, t44017: F, t44020: F, t44024: F, t44027: F, t44029: F, t44032: F, t44035: F, t44037: F, t44039: F, t44043: F, t44046: F, t44049: F, t44051: F, t44054: F) -> (F, F) {
    let t44057 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t42331 * t3582;
    let t44058 = -t44014 - t44017 + t44020 + t44024 + t44027 + t44029 + t44032 - t44035 + t44037 - t44039 - t44043 + t44046 + t44049 - t44051 - t44054 - t44057;
    (t44057, t44058)
}
