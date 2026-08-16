//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 774/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk774<F: Float>(t11346: F, t11386: F, t11395: F, t11430: F, t11880: F, t11885: F, t11891: F, t11894: F, t11896: F, t11900: F, t11907: F, t11913: F, t11918: F, t140: F, t1470: F, t1883: F, t1888: F, t1909: F, t4625: F, t4631: F, t4653: F, t4659: F, t4685: F, t479: F, t5222: F, t5231: F, t6278: F, t725: F) -> F {
    let t11921 = -F::cast_from(0.371475e-1_f64) * t725 * t11395 + F::cast_from(0.9286875e-2_f64) * t725 * t11346 - F::cast_from(0.619125e-2_f64) * t725 * t11386 + F::cast_from(0.27860625e-1_f64) * t1909 * t4653 - F::cast_from(0.1857375e-1_f64) * t1909 * t4685 + F::cast_from(0.371475e-1_f64) * t1909 * t4659 + F::cast_from(0.139303125e-1_f64) * t1909 * t4625 + F::cast_from(0.15918666666666666666e0_f64) * t6278 * t11880 - t11885 + F::cast_from(0.27860625e-1_f64) * t5222 * t1883 - F::cast_from(0.1857375e-1_f64) * t5222 * t1888 - F::cast_from(0.79593333333333333333e-1_f64) * t11891 + F::cast_from(0.26531111111111111111e-1_f64) * t11894 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t11896 - F::cast_from(0.5572125e-1_f64) * t11900 * t4631 - F::cast_from(0.27860625e-1_f64) * t5231 * t11430 - F::cast_from(0.13265555555555555556e0_f64) * t1470 * t11907 - F::cast_from(0.11791604938271604938e0_f64) * t1470 * t11913 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t11918;
    t11921
}
