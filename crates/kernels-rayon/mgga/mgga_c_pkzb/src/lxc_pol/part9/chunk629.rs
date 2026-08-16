//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 629/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk629(t2786: f64, t683: f64, t1899: f64, t1833: f64, t1905: f64, t2730: f64, t2741: f64, t1088: f64, t694: f64, t1096: f64, t702: f64, t1883: f64, t1923: f64, t1928: f64, t2755: f64, t2760: f64, t2766: f64, t2768: f64, t2772: f64, t2776: f64, t2780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2787 = t2786 * t683;
    let t2789 = 0.16081979498692535067e2_f64 * t1899 * t2787;
    let t2793 = t1905 - 0.17123333333333333333e-1_f64 * t1833 - 0.17123333333333333333e-1_f64 * t2730 + 0.5137e-1_f64 * t2741;
    let t2796 = t1088 * t694;
    let t2801 = t1096 * t702;
    let t2815 = -0.17648625e1_f64 * t2755 + 0.3529725e1_f64 * t2760 + t1923 - 0.516475e0_f64 * t1833 - 0.516475e0_f64 * t2730 + 0.1549425e1_f64 * t2741 + 0.31558125e0_f64 * t2766 + 0.6311625e0_f64 * t2768 + t1928 - 0.20839e0_f64 * t1883 - 0.20839e0_f64 * t2772 + 0.312585e0_f64 * t2776 + 0.312585e0_f64 * t2780;
    (t2787, t2789, t2793, t2796, t2801, t2815)
}
