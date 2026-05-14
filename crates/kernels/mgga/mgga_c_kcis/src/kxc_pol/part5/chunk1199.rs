//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1199/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1199<F: Float>(t3728: F, t7198: F, t12275: F, t12279: F, t12303: F, t12306: F, t16820: F, t22215: F, t22221: F, t22226: F, t22229: F, t22231: F, t5752: F, t5757: F, t1464: F, t15808: F, t2012: F) -> (F, F, F, F) {
    let t22233 = t3728 * t7198;
    let t22235 = -0.36848765432098765431e-3 * t12275 + 0.55273148148148148147e-3 * t12279 - 0.24872916666666666666e-2 * t22215 - 0.55273148148148148147e-3 * t12303 - 0.11054629629629629629e-2 * t16820 + t12306 - 0.24320185185185185185e-1 * t22221 + 0.1621345679012345679e-1 * t22226 - 0.88437037037037037034e-2 * t22229 - 0.16581944444444444444e-2 * t22231 + 0.22109259259259259259e-2 * t22233;
    let t22237 = t5752 * t5757;
    let t22238 = t1464 * t22237;
    let t22240 = t15808 * t2012;
    (t22233, t22235, t22238, t22240)
}
