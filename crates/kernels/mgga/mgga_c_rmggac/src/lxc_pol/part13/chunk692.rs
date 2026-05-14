//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 692/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk692<F: Float>(t5542: F, t7541: F, t674: F, t7244: F, t7469: F, t108: F, t4179: F, t490: F, t1223: F, t1966: F, t1968: F, t464: F, t1973: F, t214: F, t4517: F, t2007: F, t34881: F) -> (F, F, F, F, F, F, F, F) {
    let t35276 = t7541 * t5542;
    let t35277 = t35276 * t674;
    let t35285 = t7244 * t7469;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    let t35326 = t1966 * t464 * t1223 * t1968;
    let t35327 = t35326 * t1973;
    let t35331 = t1966 * t4517 * t214 * t1968;
    let t35337 = t34881 * t2007;
    (t35276, t35277, t35285, t35312, t35326, t35327, t35331, t35337)
}
