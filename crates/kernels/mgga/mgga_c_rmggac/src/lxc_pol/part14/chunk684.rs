//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 684/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk684<F: Float>(t5016: F, t7707: F, t1173: F, t674: F, t7942: F, t34884: F, t7733: F, t2185: F, t7716: F, t1997: F, t1004: F, t107: F, t490: F, t7288: F, t7494: F, t2084: F, t2134: F, t27: F, t833: F) -> (F, F, F, F, F, F, F, F) {
    let t35132 = t5016 * t7707;
    let t35146 = t7942 * t1173 * t674;
    let t35149 = t34884 * t7733;
    let t35151 = t7716 * t2185;
    let t35152 = t35151 * t1997;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35184 = t7494 * t7288;
    let t35188 = t2134 * t27 * t2084 * t833;
    (t35132, t35146, t35149, t35151, t35152, t35155, t35184, t35188)
}
