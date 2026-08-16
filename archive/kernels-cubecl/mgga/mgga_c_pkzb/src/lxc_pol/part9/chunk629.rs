//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 629/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk629<F: Float>(t2786: F, t683: F, t1899: F, t1833: F, t1905: F, t2730: F, t2741: F, t1088: F, t694: F, t1096: F, t702: F, t1883: F, t1923: F, t1928: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F, t2776: F, t2780: F) -> (F, F, F, F, F, F) {
    let t2787 = t2786 * t683;
    let t2789 = F::cast_from(0.16081979498692535067e2_f64) * t1899 * t2787;
    let t2793 = t1905 - F::cast_from(0.17123333333333333333e-1_f64) * t1833 - F::cast_from(0.17123333333333333333e-1_f64) * t2730 + F::cast_from(0.5137e-1_f64) * t2741;
    let t2796 = t1088 * t694;
    let t2801 = t1096 * t702;
    let t2815 = -F::cast_from(0.17648625e1_f64) * t2755 + F::cast_from(0.3529725e1_f64) * t2760 + t1923 - F::cast_from(0.516475e0_f64) * t1833 - F::cast_from(0.516475e0_f64) * t2730 + F::cast_from(0.1549425e1_f64) * t2741 + F::cast_from(0.31558125e0_f64) * t2766 + F::cast_from(0.6311625e0_f64) * t2768 + t1928 - F::cast_from(0.20839e0_f64) * t1883 - F::cast_from(0.20839e0_f64) * t2772 + F::cast_from(0.312585e0_f64) * t2776 + F::cast_from(0.312585e0_f64) * t2780;
    (t2787, t2789, t2793, t2796, t2801, t2815)
}
