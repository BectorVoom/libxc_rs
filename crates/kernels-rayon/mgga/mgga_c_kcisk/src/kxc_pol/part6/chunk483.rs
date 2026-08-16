//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 483/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk483(t4663: f64, t677: f64, t4636: f64, t1643: f64, t583: f64, t573: f64, t571: f64, t1379: f64, t311: f64, t579: f64, t1774: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4664 = t4663 * t677;
    let t4676 = 0.55033333333333333333e-2_f64 * t4636;
    let t4691 = 0.23744444444444444444e-1_f64 * t4636;
    let t4702 = t1643 * t583;
    let t4703 = 1.0_f64 / t4702;
    let t4704 = t573 * t4703;
    let t4711 = 0.39862222222222222223e0_f64 * t4636;
    let t4716 = 1.0_f64/f64::sqrt(t571);
    let t4722 = t311 * t1379 * t579;
    let t4723 = 0.13692777777777777778e0_f64 * t4722;
    let t4726 = t79 * t1774;
    (t4664, t4676, t4691, t4703, t4704, t4711, t4716, t4722, t4723, t4726)
}
