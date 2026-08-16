//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1198/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1198(t1861: f64, t667: f64, t7360: f64, t1867: f64, t7365: f64, t2754: f64, t5540: f64, t7370: f64, t2765: f64, t20759: f64, t20762: f64, t20765: f64, t20769: f64, t20773: f64, t20777: f64, t20781: f64, t20789: f64, t20791: f64, t20794: f64, t20797: f64, t20800: f64, t20803: f64, t20806: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20809 = t1861 * t7360 * t667;
    let t20811 = t7365 * t1867;
    let t20813 = t2754 * t5540;
    let t20815 = t7370 * t1867;
    let t20817 = t2765 * t5540;
    let t20819 = -0.49671e0_f64 * t20759 - 0.99342e0_f64 * t20762 - 0.49671e0_f64 * t20765 + 0.248355e0_f64 * t20769 + 0.745065e0_f64 * t20773 + 0.745065e0_f64 * t20777 + 0.248355e0_f64 * t20781 + 0.16504875e0_f64 * t20789 + 0.258925e1_f64 * t20791 + 0.58258125e1_f64 * t20794 - 0.1237865625e0_f64 * t20797 - 0.485484375e1_f64 * t20800 + 0.6189328125e-1_f64 * t20803 + 0.247573125e0_f64 * t20806 - 0.3883875e1_f64 * t20809 - 0.3883875e1_f64 * t20811 - 0.1294625e1_f64 * t20813 + 0.247573125e0_f64 * t20815 + 0.82524375e-1_f64 * t20817;
    (t20809, t20811, t20813, t20815, t20817, t20819)
}
