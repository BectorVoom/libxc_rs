//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 774/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk774(t11346: f64, t11386: f64, t11395: f64, t11430: f64, t11880: f64, t11885: f64, t11891: f64, t11894: f64, t11896: f64, t11900: f64, t11907: f64, t11913: f64, t11918: f64, t140: f64, t1470: f64, t1883: f64, t1888: f64, t1909: f64, t4625: f64, t4631: f64, t4653: f64, t4659: f64, t4685: f64, t479: f64, t5222: f64, t5231: f64, t6278: f64, t725: f64) -> f64 {
    let t11921 = -0.371475e-1_f64 * t725 * t11395 + 0.9286875e-2_f64 * t725 * t11346 - 0.619125e-2_f64 * t725 * t11386 + 0.27860625e-1_f64 * t1909 * t4653 - 0.1857375e-1_f64 * t1909 * t4685 + 0.371475e-1_f64 * t1909 * t4659 + 0.139303125e-1_f64 * t1909 * t4625 + 0.15918666666666666666e0_f64 * t6278 * t11880 - t11885 + 0.27860625e-1_f64 * t5222 * t1883 - 0.1857375e-1_f64 * t5222 * t1888 - 0.79593333333333333333e-1_f64 * t11891 + 0.26531111111111111111e-1_f64 * t11894 - 0.39796666666666666666e-1_f64 * t140 * t479 * t11896 - 0.5572125e-1_f64 * t11900 * t4631 - 0.27860625e-1_f64 * t5231 * t11430 - 0.13265555555555555556e0_f64 * t1470 * t11907 - 0.11791604938271604938e0_f64 * t1470 * t11913 - 0.79593333333333333333e-1_f64 * t1470 * t11918;
    t11921
}
