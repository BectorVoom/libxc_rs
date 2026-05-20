//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2254/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2254<F: Float>(t17528: F, t3594: F, t1214: F, t4186: F, t5296: F, t1042: F, t1469: F, t3584: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17529 = t3594 * t17528;
    let t17534 = t4186 * t1214;
    let t17535 = t5296 * t17534;
    let t17536 = t1042 * t17535;
    let t17539 = t1469 * t3584;
    let t17540 = t5296 * t17539;
    let t17541 = t1042 * t17540;
    let t17544 = t3172 * t5286;
    let t17546 = F::cast_from(0.28582678745379824648e-3_f64) * t1247 * t17544;
    let t17547 = t3707 * t5292;
    (t17529, t17534, t17535, t17536, t17539, t17540, t17541, t17544, t17546, t17547)
}
