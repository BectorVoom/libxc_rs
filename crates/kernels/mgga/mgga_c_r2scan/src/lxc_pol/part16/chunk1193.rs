//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1193/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1193<F: Float>(t1592: F, t29270: F, t3308: F, t2196: F, t29274: F, t1054: F, t5108: F, t8760: F, t6583: F, t8764: F, t8769: F, t37891: F, t37903: F, t39793: F, t39824: F, t39826: F, t39828: F, t41570: F) -> F {
    let t43248 = t1592 * t3308 * t29270;
    let t43251 = t2196 * t3308 * t29274;
    let t43256 = t5108 * t1054 * t8760;
    let t43259 = t6583 * t1054 * t8764;
    let t43262 = t5108 * t1054 * t8769;
    let t43264 = F::new(0.13002332610081402845e0) * t43248 + F::new(0.5200933044032561138e0) * t43251 + t39793 - F::new(0.42683466926433871472e0) * t37891 - F::new(0.15573871527278325618e-1) * t37903 + t41570 - F::new(0.2600466522016280569e0) * t43256 - F::new(0.17336443480108537126e0) * t43259 - F::new(0.2600466522016280569e0) * t43262 - t39824 - t39826 - t39828;
    t43264
}
