//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 599/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk599<F: Float>(t260: F, t604: F, t67: F, t41: F, t4971: F, t1001: F, t167: F, t2689: F, t1049: F, t116: F, t1596: F, t4350: F, t2028: F, t5439: F, t3182: F, t1065: F, t3462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7567 = t260 * t67 * t604;
    let t7568 = t41 * t4971;
    let t9345 = t167 * t1001;
    let t9352 = t2689 * t1001;
    let t9355 = t116 * t1049;
    let t9517 = t4350 * t1596;
    let t9726 = t5439 * t2028;
    let t10328 = 6.0 * t3182;
    let t10329 = t1065 * t3462;
    (t7567, t7568, t9345, t9352, t9355, t9517, t9726, t10328, t10329)
}
