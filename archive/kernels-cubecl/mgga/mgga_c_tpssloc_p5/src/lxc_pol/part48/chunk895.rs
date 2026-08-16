//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 895/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk895<F: Float>(t1373: F, t254: F, t1324: F, t6875: F, t8944: F, t671: F, t7039: F, t2035: F, t2363: F, t2094: F, t40611: F, t12461: F, t7216: F) -> (F, F, F, F, F, F, F) {
    let t90665 = t1373 * t254;
    let t91505 = t1324 * t254;
    let t91669 = t6875 * t8944;
    let t91854 = t7039 * t671;
    let t91857 = t2035 * t2363;
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    (t90665, t91505, t91669, t91854, t91857, t92169, t92200)
}
