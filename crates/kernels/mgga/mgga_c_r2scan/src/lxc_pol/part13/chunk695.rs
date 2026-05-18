//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 695/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk695<F: Float>(t234: F, t5300: F, t1814: F, t732: F, t1813: F, t1841: F, t148: F, t1683: F, t5245: F, t22: F, t502: F, t1712: F) -> (F, F, F, F, F, F) {
    let t5302 = F::new(0.6233709278045326953e3) * t234 * t5300;
    let t5303 = t732 * t1814;
    let t5305 = t1841 * t1813;
    let t5307 = F::new(0.51947577317044391277e2) * t234 * t5305;
    let t5308 = t148 * t1683;
    let t5309 = t5308 * t5245;
    let t5311 = t22 * t502;
    let t5312 = t1712 * t5311;
    (t5302, t5303, t5307, t5309, t5311, t5312)
}
