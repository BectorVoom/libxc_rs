//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1312/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1312<F: Float>(t225: F, t23202: F, t6556: F, t81632: F, t23012: F, t6573: F, t1883: F, t82045: F, t23164: F, t6555: F, t82133: F, t23270: F, t2379: F, t25038: F, t857: F, t865: F) -> (F, F, F, F, F, F) {
    let t82197 = t23202 * t225;
    let t82209 = t81632 * t6556;
    let t82211 = t23012 * t6573;
    let t82218 = t82045 * t1883;
    let t82219 = F::cast_from(0.27720185200590482541e0_f64) * t82218;
    let t82221 = t23164 * t82133 * t6555;
    let t82228 = t25038 * t23270 * t857 * t2379 * t865;
    (t82197, t82209, t82211, t82219, t82221, t82228)
}
