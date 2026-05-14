//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1169/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1169<F: Float>(t2007: F, t554: F, t8508: F, t1993: F, t6461: F, t8451: F, t8488: F, t8463: F, t8474: F, t3015: F, t6432: F, t125: F, t8428: F, t13: F, t21775: F, t2986: F) -> (F, F, F, F, F, F, F) {
    let t25403 = t554 * t2007 * t8508;
    let t25406 = t1993 * t6461 * t8451;
    let t25421 = t554 * t2007 * t8488;
    let t25424 = t554 * t8474 * t8463;
    let t25428 = t554 * t6432 * t3015;
    let t25430 = t8428 * t125;
    let t25444 = t21775 * t13 * t2986;
    (t25403, t25406, t25421, t25424, t25428, t25430, t25444)
}
