//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 651/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk651<F: Float>(t3: F, t40: F, t5244: F, t5239: F, t1725: F, t58: F, t423: F, t170: F, t1727: F, t597: F, t1823: F, t732: F, t1818: F, t712: F, t1822: F, t234: F) -> (F, F, F, F, F, F, F, F) {
    let t5245 = t3 * t40;
    let t5246 = t5244 * t5245;
    let t5248 = 0.42340699333333333333e-2 * t5239 * t5246;
    let t5249 = t1725 * t58;
    let t5250 = t5249 * t423;
    let t5251 = t170 * t1727;
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5258 = t732 * t1823;
    let t5260 = t1818 * t712;
    let t5261 = t5260 * t1822;
    let t5263 = 0.30762056574649219973e4 * t234 * t5261;
    (t5245, t5246, t5248, t5249, t5252, t5253, t5258, t5263)
}
