//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1128/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1128<F: Float>(t10361: F, t942: F, t10283: F, t10297: F, t10300: F, t10306: F, t1246: F, t1256: F, t3247: F, t3255: F, t3279: F, t3904: F, t3910: F, t3929: F, t411: F, t415: F, t938: F, t952: F) -> (F, F) {
    let t10362 = t942 * t10361;
    let t10365 = 0.65854491829355115987e0 * t10283 * t415 - 0.65854491829355115987e0 * t3904 * t952 - 0.13170898365871023197e1 * t3247 * t1256 + 0.26341796731742046394e1 * t1246 * t3255 - 0.13170898365871023197e1 * t1246 * t3279 + 0.13170898365871023197e1 * t938 * t3910 - 0.39512695097613069591e1 * t411 * t10297 + 0.26341796731742046394e1 * t411 * t10300 - 0.65854491829355115987e0 * t938 * t3929 + 0.13170898365871023197e1 * t411 * t10306 - 0.65854491829355115987e0 * t411 * t10362;
    (t10362, t10365)
}
