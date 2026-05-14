//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 944/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk944<F: Float>(t1543: F, t2567: F, t113: F, t2526: F, t494: F, t545: F, t7600: F, t146: F, t6091: F, t978: F, t24877: F, t2573: F, t481: F, t1550: F, t938: F, t2145: F, t2832: F) -> (F, F, F, F, F, F, F, F) {
    let t26186 = t2567 * t1543;
    let t26274 = t2526 * t494 * t113;
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t26307 = t24877 * t113;
    let t26314 = t2573 * t481;
    let t26997 = t938 * t1550 * t113;
    let t27067 = t146 * t2145 * t2832;
    (t26186, t26274, t26278, t26282, t26307, t26314, t26997, t27067)
}
