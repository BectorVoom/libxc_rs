//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 641/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk641(t5283: f64, t718: f64, t41: f64, t719: f64, t1646: f64, t725: f64, t707: f64, t4594: f64, t702: f64, t1797: f64, t5061: f64, t5320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7315 = t5283 * t718;
    let t7316 = t41 * t719;
    let t7349 = t725 * t1646;
    let t7360 = t725 * t707;
    let t7370 = t4594 * t702;
    let t7378 = t1797 * t702;
    let t7429 = t5061 * t5320;
    (t7315, t7316, t7349, t7360, t7370, t7378, t7429)
}
