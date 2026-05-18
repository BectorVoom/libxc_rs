//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1182/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1182<F: Float>(t37031: F, t8367: F, t3366: F, t8355: F, t23495: F, t3363: F, t1035: F, t1339: F, t352: F, t1343: F, t3675: F, t12025: F) -> (F, F, F, F, F, F) {
    let t40842 = t37031 * t8367;
    let t40844 = t8355 * t3366;
    let t40848 = t23495 * t3363;
    let t41058 = t1035 * t1339 * t352;
    let t41065 = t3675 * t1343;
    let t41116 = F::new(45.0) / F::new(32.0) * t12025;
    (t40842, t40844, t40848, t41058, t41065, t41116)
}
