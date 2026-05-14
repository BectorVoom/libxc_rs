//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1331/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1331<F: Float>(t1234: F, t1536: F, t1541: F, t1544: F, t1547: F, t2498: F, t2504: F, t2527: F, t481: F, t5043: F, t5055: F, t5059: F, t5062: F, t6334: F, t7088: F, t7175: F, t7180: F, t7181: F, t7184: F, t8668: F, t915: F, t917: F) -> (F,) {
    let t25086 = -36.0 * t1541 * t2504 * t481 * t7088 - 36.0 * t1234 * t2504 * t7184 + 180.0 * t2504 * t6334 * t7180 + 9.0 * t1536 * t2527 - 36.0 * t1544 * t2498 + 9.0 * t1547 * t2498 + 3.0 * t5043 * t917 + 60.0 * t5055 * t915 - 36.0 * t5059 * t8668 + 3.0 * t5062 * t915 + 180.0 * t7175 * t7181;
    (t25086,)
}
