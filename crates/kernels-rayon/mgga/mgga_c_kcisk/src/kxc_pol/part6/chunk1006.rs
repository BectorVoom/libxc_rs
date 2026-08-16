//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1006/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1006(t30294: f64, t3661: f64, t26: f64, t1186: f64, t30273: f64, t30233: f64, t4271: f64, t12: f64) -> (f64, f64, f64) {
    let t30581 = t3661 * t30294;
    let t30582 = t26 * t30581;
    let t30584 = t1186 * t30273;
    let t30585 = t26 * t30584;
    let t30591 = t4271 * t30233;
    let t30592 = t12 * t30591;
    (t30582, t30585, t30592)
}
