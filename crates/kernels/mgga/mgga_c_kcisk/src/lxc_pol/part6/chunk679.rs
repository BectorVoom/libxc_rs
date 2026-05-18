//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 679/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk679<F: Float>(t10933: F, t606: F, t11032: F, t1848: F, t641: F, t916: F, t5014: F, t5030: F, t1691: F, t604: F, t4825: F, t667: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11040 = F::new(28.0) / F::new(27.0) * t10933;
    let t11056 = F::new(1.0)/pow_3_2::<f64>(t606);
    let t11091 = F::new(0.93932222222222222223e0) * t10933;
    let t11092 = F::new(0.73586666666666666667e0) * t11032;
    let t11105 = F::new(0.55403703703703703703e-1) * t10933;
    let t11153 = F::new(1.0) / t641 / t916 / t1848;
    let t11179 = t5014 * t5030;
    let t11195 = t1691 * t1691;
    let t11196 = F::new(1.0) / t11195;
    let t11197 = t604 * t11196;
    let t11200 = F::new(1.0) / t4825 / t667;
    (t11040, t11056, t11091, t11092, t11105, t11153, t11179, t11197, t11200)
}
