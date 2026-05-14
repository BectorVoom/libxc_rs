//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 774/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk774<F: Float>(t2310: F, t34881: F, t35384: F, t2313: F, t34855: F, t674: F, t7411: F, t1240: F, t236: F, t618: F, t7230: F, t7231: F, t2305: F, t35326: F, t7371: F, t8577: F) -> (F, F, F, F, F, F) {
    let t39295 = t34881 * t2310;
    let t39297 = t35384 * t2310;
    let t39300 = t2313 * t34855 * t674;
    let t39301 = t39300 * t7411;
    let t39306 = t7230 * t7231 * t236 * t618 * t1240;
    let t39308 = t35326 * t2305;
    let t39310 = t8577 * t7371;
    (t39295, t39297, t39301, t39306, t39308, t39310)
}
