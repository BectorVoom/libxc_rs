//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 621/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk621<F: Float>(t2786: F, t683: F, t1899: F, t1833: F, t1905: F, t2730: F, t2741: F, t1088: F, t694: F, t1096: F, t702: F, t1883: F, t1923: F, t1928: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F, t2776: F, t2780: F) -> (F, F, F, F, F, F) {
    let t2787 = t2786 * t683;
    let t2789 = 0.16081979498692535067e2 * t1899 * t2787;
    let t2793 = t1905 - 0.17123333333333333333e-1 * t1833 - 0.17123333333333333333e-1 * t2730 + 0.5137e-1 * t2741;
    let t2796 = t1088 * t694;
    let t2801 = t1096 * t702;
    let t2815 = -0.17648625e1 * t2755 + 0.3529725e1 * t2760 + t1923 - 0.516475e0 * t1833 - 0.516475e0 * t2730 + 0.1549425e1 * t2741 + 0.31558125e0 * t2766 + 0.6311625e0 * t2768 + t1928 - 0.20839e0 * t1883 - 0.20839e0 * t2772 + 0.312585e0 * t2776 + 0.312585e0 * t2780;
    (t2787, t2789, t2793, t2796, t2801, t2815)
}
