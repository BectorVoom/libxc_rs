//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1430/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1430<F: Float>(t1604: F, t33075: F, t2155: F, t22962: F, t34345: F, t34582: F, t8088: F, t10069: F, t19978: F, t21003: F, t22959: F, t259: F, t2600: F, t27198: F, t29517: F, t31120: F, t31144: F, t31156: F, t31158: F, t32319: F, t34321: F, t360: F, t571: F, t6132: F, t6134: F, t6149: F, t8029: F, t8820: F) -> (F,) {
    let t34674 = t1604 * t33075;
    let t34677 = t2155 * t22962 * t34345;
    let t34680 = t2155 * t8088 * t34582;
    let t34682 = -0.52396431978519890151e-1 * t31120 + 0.7801399566048841707e0 * t21003 * t360 * t32319 * t19978 - 0.26004665220162805689e0 * t6132 * t360 * t32319 * t6134 + 0.13002332610081402845e0 * t6149 * t10069 + 0.2600466522016280569e0 * t571 * t29517 * t259 * t2600 + 0.76830240467580968652e0 * t31144 - 0.7801399566048841707e0 * t8029 * t360 * t8820 * t34321 + 0.98781737744032673979e-1 * t27198 - 0.12225834128321307702e1 * t31156 - 0.35126785941778018867e0 * t31158 - t22959 + 0.27439371595564631661e-2 * t34674 - 0.5854464323629669811e-1 * t34677 + 0.29272321618148349055e-1 * t34680;
    (t34682,)
}
