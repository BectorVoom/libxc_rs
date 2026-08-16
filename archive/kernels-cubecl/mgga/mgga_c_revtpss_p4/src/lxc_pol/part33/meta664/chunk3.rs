//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2166/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2166<F: Float>(t1444: F, t6844: F, t30095: F, t689: F, t25904: F, t25899: F, t1903: F, t543: F, t5658: F, t14224: F, t1882: F, t25930: F, t25931: F, t27837: F, t27846: F, t27868: F, t27960: F, t30055: F, t30105: F, t7295: F, t7296: F, t7301: F, t94635: F, t94648: F, t94716: F, t97823: F, t97825: F, t97838: F, t97875: F) -> F {
    let t108244 = t6844 * t1444;
    let t108248 = t30095 * t689;
    let t108249 = t25904 * t108248;
    let t108251 = t25899 * t108248;
    let t108259 = t1903 * t5658 * t543;
    let t108270 = -F::cast_from(0.17347256376410398924e1_f64) * t25930 * t94716 * t30105 - F::cast_from(0.14634331517634470219e-1_f64) * t97823 + F::cast_from(0.26019841438354088051e-1_f64) * t97825 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t97875 * t14224 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t108244 - F::cast_from(0.72280234901709995518e-2_f64) * t108249 + F::cast_from(0.12851425765524037203e-1_f64) * t108251 - F::cast_from(0.17135234354032049604e-1_f64) * t94635 + t94648 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t30055 * t1444 + t97838 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t25931 * t108259 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t27960 * t1882 * t543 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t27846;
    t108270
}
