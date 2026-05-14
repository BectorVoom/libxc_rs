//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1359/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1359<F: Float>(t113350: F, t6373: F, t1513: F, t25308: F, t32269: F, t34863: F, t27213: F, t32260: F, t109287: F, t34858: F, t8244: F, t27210: F, t9497: F, t113478: F, t9836: F, t14188: F, t8268: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119833 = t113350 * t6373;
    let t119835 = t25308 * t1513;
    let t119837 = t32269 * t34863;
    let t119839 = t32260 * t27213;
    let t119841 = t109287 * t34858;
    let t119843 = t32269 * t8244;
    let t119845 = t9497 * t27210;
    let t119847 = t113478 * t9836;
    let t119849 = t14188 * t8268;
    (t119833, t119835, t119837, t119839, t119841, t119843, t119845, t119847, t119849)
}
