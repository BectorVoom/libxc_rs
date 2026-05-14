//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1383/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1383<F: Float>(t20286: F, t20705: F, t20773: F, t22768: F, t22770: F, t22775: F, t22778: F, t2567: F, t26218: F, t26221: F, t26231: F, t26234: F, t26238: F, t26244: F, t360: F, t6428: F, t7461: F, t7578: F, t7585: F, t8172: F) -> (F,) {
    let t26246 = 0.17465477326173296717e-1 * t26218 + 0.34930954652346593433e-1 * t26221 - 0.15602799132097683414e1 * t7461 * t360 * t2567 * t6428 + 0.47709005517312117571e-2 * t22768 - 0.69345773920434148506e0 * t22770 - 0.10401866088065122276e1 * t22775 + 0.5141876673348786705e0 * t22778 - 0.20803732176130244552e1 * t26231 - 0.48787202696913915093e-3 * t26234 - 0.7801399566048841707e0 * t20286 * t8172 + 0.92480845007273388189e0 * t26238 - 0.15602799132097683414e1 * t20705 * t7578 - 0.78013995660488417068e0 * t20773 * t7585 - 0.20803732176130244552e1 * t26244;
    (t26246,)
}
