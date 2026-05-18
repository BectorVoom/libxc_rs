//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 385/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk385<F: Float>(t1542: F, t1543: F, t1234: F, t490: F, t109: F, t111: F, t1536: F, t486: F, t491: F) -> (F, F, F) {
    let t1544 = t1542 * t1543;
    let t1547 = t490 * t1234;
    let t1550 = -F::new(12.0) * t109 * t1544 + F::new(3.0) * t109 * t1547 - t1536 * t111 + F::new(6.0) * t486 * t491;
    (t1544, t1547, t1550)
}
