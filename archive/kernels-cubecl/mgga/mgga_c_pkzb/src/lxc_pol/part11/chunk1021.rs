//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1021/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1021<F: Float>(t11260: F, t852: F, t833: F, t11233: F, t6201: F, t6199: F, t11166: F, t2281: F, t11155: F, t11185: F, t11187: F, t11191: F, t11196: F, t11198: F, t11200: F, t11207: F, t11211: F, t6249: F, t6256: F, t7950: F, t7955: F, t9782: F, t9819: F, t9826: F) -> (F, F, F, F, F, F) {
    let t11261 = t11260 * t852;
    let t11263 = F::cast_from(1.0_f64) * t833 * t11261;
    let t11264 = t11233 * t6201;
    let t11266 = F::cast_from(0.51726012919273400301e3_f64) * t6199 * t11264;
    let t11269 = t11166 * t2281;
    let t11286 = F::cast_from(0.264729375e1_f64) * t11185 - F::cast_from(0.52945875e1_f64) * t11187 + F::cast_from(0.3529725e1_f64) * t11191 - t6249 + F::cast_from(0.20659e1_f64) * t7955 - F::cast_from(0.1549425e1_f64) * t9782 + F::cast_from(0.1549425e1_f64) * t11155 - F::cast_from(0.157790625e0_f64) * t11196 + F::cast_from(0.94674375e0_f64) * t11198 + F::cast_from(0.6311625e0_f64) * t11200 - t6256 + F::cast_from(0.104195e1_f64) * t7950 - F::cast_from(0.62517e0_f64) * t9819 - F::cast_from(0.62517e0_f64) * t9826 + F::cast_from(0.937755e0_f64) * t11207 + F::cast_from(0.312585e0_f64) * t11211;
    (t11261, t11263, t11264, t11266, t11269, t11286)
}
