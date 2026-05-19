//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1008/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1008<F: Float>(t25640: F, t36: F, t5163: F, t25529: F, t40893: F, t3826: F, t40897: F, t38745: F, t3810: F, t39670: F, t25518: F, t6444: F, t8704: F) -> (F, F, F, F, F, F, F) {
    let t41165 = t25640 * t36;
    let t41166 = t41165 * t5163;
    let t41168 = t25529 * t40893;
    let t41170 = t3826 * t40897;
    let t41171 = F::cast_from(0.10620923284048465071e-1_f64) * t41170;
    let t41172 = t3826 * t38745;
    let t41174 = t3810 * t39670;
    let t41176 = t25518 * t36;
    let t41177 = t41176 * t5163;
    let t41179 = t6444 * t8704;
    (t41166, t41168, t41171, t41172, t41174, t41177, t41179)
}
