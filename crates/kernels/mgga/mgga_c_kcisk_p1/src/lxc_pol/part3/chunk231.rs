//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 231/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk231<F: Float>(t1001: F, t116: F, t982: F, t979: F, t136: F, t852: F, t856: F, t934: F, t939: F, t977: F, t218: F, t217: F) -> (F, F, F, F, F, F) {
    let t1002 = t116 * t1001;
    let t1003 = t982 * t1002;
    let t1004 = t979 * t1003;
    let t1006 = t852 * t136 - F::cast_from(0.193e0_f64) * t856 * t934 - F::cast_from(0.13265555555555555555e-1_f64) * t939 + F::cast_from(0.99491666666666666664e-2_f64) * t977 - F::cast_from(0.99491666666666666664e-2_f64) * t1004;
    let t1007 = t1006 * t218;
    let t1008 = t217 * t217;
    (t1002, t1003, t1004, t1006, t1007, t1008)
}
