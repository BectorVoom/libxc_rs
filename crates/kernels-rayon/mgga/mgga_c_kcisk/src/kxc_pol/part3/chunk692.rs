//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 692/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk692(t604: f64, t10777: f64, t1783: f64, t1310: f64, t25: f64, t5033: f64, t1773: f64, t1769: f64, t4984: f64, t1765: f64, t4995: f64, t657: f64, t963: f64) -> (f64, f64, f64, f64, f64) {
    let t659 = 0.0_f64 < t604;
    let t10779 = piecewise3(t659, t10777, -t10777);
    let t10780 = t1783 * t10779;
    let t10781 = t1310 * t10780;
    let t10784 = t25 * t5033;
    let t10785 = t1773 * t10784;
    let t10787 = t4984 * t1769;
    let t10789 = t1765 * t4995;
    let t10791 = t963 * t657;
    (t10781, t10785, t10787, t10789, t10791)
}
