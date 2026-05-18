//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 969/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk969<F: Float>(t1445: F, t1486: F, t4205: F, t13320: F, t4204: F, t4203: F, t4223: F, t4226: F, t13949: F, t4231: F, t4230: F, t1492: F, t4210: F) -> (F, F, F, F, F) {
    let t14304 = t1486 * t1445;
    let t14305 = t14304 * t4205;
    let t14307 = t4204 * t13320;
    let t14308 = t4203 * t14307;
    let t14310 = t4223 * t4226;
    let t14312 = t4231 * t13949;
    let t14313 = t4230 * t14312;
    let t14315 = t1492 * t4210;
    (t14305, t14308, t14310, t14313, t14315)
}
