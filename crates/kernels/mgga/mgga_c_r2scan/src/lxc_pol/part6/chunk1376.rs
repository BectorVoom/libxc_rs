//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1376/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1376<F: Float>(t22729: F, t22732: F, t22734: F, t22737: F, t22741: F, t22745: F, t2567: F, t26066: F, t26097: F, t26106: F, t26109: F, t26116: F, t26119: F, t26124: F, t360: F, t5109: F, t6139: F, t6198: F, t7512: F) -> (F,) {
    let t26126 = -0.52396431978519890151e-1 * t26097 + 0.98781737744032673981e-1 * t22729 + 0.1047928639570397803e0 * t22732 + 0.4075278042773769234e0 * t22734 - 0.12225834128321307702e1 * t22737 + 0.34930954652346593433e-1 * t22741 + 0.59329162131926993721e1 * t26106 - t26109 + 0.11524536070137145298e1 * t22745 - 0.7801399566048841707e0 * t7512 * t360 * t2567 * t6198 + t26116 + t26119 - 0.23404198698146525121e1 * t6139 * t5109 * t26066 + 0.20803732176130244552e1 * t26124;
    (t26126,)
}
