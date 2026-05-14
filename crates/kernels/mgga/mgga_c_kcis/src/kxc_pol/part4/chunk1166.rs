//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1166/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1166<F: Float>(t16685: F, t5653: F, t4170: F, t4160: F, t11425: F, t556: F, t16694: F, t5661: F, t1404: F, t4035: F, t1961: F, t833: F, t1419: F, t12048: F, t5796: F, t1401: F, t5808: F) -> (F, F, F, F, F, F) {
    let t17005 = t5653 * t16685;
    let t17006 = t4170 * t17005;
    let t17007 = t4160 * t17006;
    let t17009 = t556 * t11425;
    let t17010 = t17009 * t16694;
    let t17011 = t4170 * t17010;
    let t17012 = t5661 * t17011;
    let t17019 = t1404 * t4035;
    let t17020 = t1961 * t833;
    let t17021 = t17020 * t1419;
    let t17024 = t12048 * t5796;
    let t17027 = 0.93706135855523581992e-2 * t1401 * t5808;
    (t17007, t17012, t17019, t17021, t17024, t17027)
}
