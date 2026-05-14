//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1464/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1464<F: Float>(t10408: F, t10550: F, t23240: F, t23241: F, t23244: F, t23245: F, t23296: F, t5016: F, t7031: F, t7032: F, t9575: F, t9579: F, t10553: F, t10556: F, t23249: F, t23297: F, t23306: F, t23307: F, t7051: F, t9582: F, t9584: F, t9587: F, t9591: F, t9915: F, t9916: F) -> (F, F) {
    let t35302 = 6.0 * t9575 - t23240 + t23241 + t5016 - t23244 - 18.0 * t9579 + t10408 + t23245 + t10550 - t23296 + 9.0 * t7031 + 0.21973736767207854065e-2 * t7032;
    let t35308 = -t23297 - t23249 + 3.0 * t9582 + 18.0 * t9584 + 36.0 * t9587 + t10553 - 72.0 * t7051 + t10556 - t9915 - 9.0 * t9591 - t23306 - t23307 + t9916;
    (t35302, t35308)
}
