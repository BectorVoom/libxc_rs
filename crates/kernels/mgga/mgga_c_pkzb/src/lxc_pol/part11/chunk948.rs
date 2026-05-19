//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 948/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk948<F: Float>(t10361: F, t942: F, t10283: F, t10297: F, t10300: F, t10306: F, t1246: F, t1256: F, t3247: F, t3255: F, t3279: F, t3904: F, t3910: F, t3929: F, t411: F, t415: F, t938: F, t952: F) -> (F, F) {
    let t10362 = t942 * t10361;
    let t10365 = F::cast_from(0.65854491829355115987e0_f64) * t10283 * t415 - F::cast_from(0.65854491829355115987e0_f64) * t3904 * t952 - F::cast_from(0.13170898365871023197e1_f64) * t3247 * t1256 + F::cast_from(0.26341796731742046394e1_f64) * t1246 * t3255 - F::cast_from(0.13170898365871023197e1_f64) * t1246 * t3279 + F::cast_from(0.13170898365871023197e1_f64) * t938 * t3910 - F::cast_from(0.39512695097613069591e1_f64) * t411 * t10297 + F::cast_from(0.26341796731742046394e1_f64) * t411 * t10300 - F::cast_from(0.65854491829355115987e0_f64) * t938 * t3929 + F::cast_from(0.13170898365871023197e1_f64) * t411 * t10306 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t10362;
    (t10362, t10365)
}
