//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 564/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk564(t1379: f64, t311: f64, t579: f64, t1660: f64, t827: f64, t1774: f64, t79: f64, t4640: f64, t26: f64, t1659: f64, t4644: f64, t4648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4722 = t311 * t1379 * t579;
    let t4723 = 0.13692777777777777778e0_f64 * t4722;
    let t4724 = t827 * t1660;
    let t4726 = t79 * t1774;
    let t4727 = t4726 * t4640;
    let t4728 = t26 * t4727;
    let t4730 = t1659 * t4644;
    let t4731 = t26 * t4730;
    let t4733 = t1659 * t4648;
    (t4722, t4723, t4724, t4726, t4727, t4728, t4730, t4731, t4733)
}
