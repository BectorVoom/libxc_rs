//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 757/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk757<F: Float>(t5053: F, t5054: F, t1541: F, t481: F, t1234: F, t490: F, t4933: F, t109: F, t111: F, t1536: F, t1544: F, t1547: F, t2504: F, t486: F, t491: F, t5043: F) -> (F, F, F, F) {
    let t5055 = t5053 * t5054;
    let t5058 = t1541 * t481;
    let t5059 = t5058 * t1234;
    let t5062 = t490 * t4933;
    let t5065 = 60.0 * t109 * t5055 + 3.0 * t109 * t5062 - t5043 * t111 + 9.0 * t1536 * t491 - 36.0 * t486 * t1544 + 9.0 * t486 * t1547 - 36.0 * t2504 * t5059;
    (t5055, t5059, t5062, t5065)
}
