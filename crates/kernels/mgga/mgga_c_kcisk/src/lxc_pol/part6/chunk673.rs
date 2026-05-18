//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 673/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk673<F: Float>(t10568: F, t5005: F, t79: F, t311: F, t3841: F, t579: F, t571: F, t574: F, t581: F, t4786: F, t596: F, t1675: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10569 = F::new(0.55403703703703703703e-1) * t10568;
    let t10621 = t79 * t5005;
    let t10639 = F::new(0.93932222222222222223e0) * t10568;
    let t10641 = t311 * t3841 * t579;
    let t10642 = F::new(0.36793333333333333333e0) * t10641;
    let t10649 = F::new(28.0) / F::new(27.0) * t10568;
    let t10663 = F::new(1.0)/pow_3_2::<f64>(t571);
    let t10671 = F::new(1.0) / t574 / t581 / F::new(4.0);
    let t10690 = F::new(1.0) / t4786 / t596;
    let t10696 = F::new(1.0) / t4786 / t1675;
    (t10569, t10621, t10639, t10641, t10642, t10649, t10663, t10671, t10690, t10696)
}
