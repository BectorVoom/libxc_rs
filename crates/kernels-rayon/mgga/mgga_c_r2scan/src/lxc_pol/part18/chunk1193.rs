//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1193/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1193(t1592: f64, t29270: f64, t3308: f64, t2196: f64, t29274: f64, t1054: f64, t5108: f64, t8760: f64, t6583: f64, t8764: f64, t8769: f64, t37891: f64, t37903: f64, t39793: f64, t39824: f64, t39826: f64, t39828: f64, t41570: f64) -> f64 {
    let t43248 = t1592 * t3308 * t29270;
    let t43251 = t2196 * t3308 * t29274;
    let t43256 = t5108 * t1054 * t8760;
    let t43259 = t6583 * t1054 * t8764;
    let t43262 = t5108 * t1054 * t8769;
    let t43264 = 0.13002332610081402845e0_f64 * t43248 + 0.5200933044032561138e0_f64 * t43251 + t39793 - 0.42683466926433871472e0_f64 * t37891 - 0.15573871527278325618e-1_f64 * t37903 + t41570 - 0.2600466522016280569e0_f64 * t43256 - 0.17336443480108537126e0_f64 * t43259 - 0.2600466522016280569e0_f64 * t43262 - t39824 - t39826 - t39828;
    t43264
}
