//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1196/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1196<F: Float>(t8244: F, t9491: F, t6332: F, t7906: F, t6317: F, t8010: F, t9497: F, t4204: F, t7831: F, t25308: F, t500: F, t2275: F, t5886: F, t2279: F, t5606: F, t3785: F, t8268: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34861 = t9491 * t8244;
    let t34863 = t6332 * t7906;
    let t34864 = t9491 * t34863;
    let t34866 = t6317 * t8010;
    let t34867 = t9497 * t34866;
    let t34869 = t4204 * t7831;
    let t34870 = t9497 * t34869;
    let t34872 = t25308 * t500;
    let t34874 = t5886 * t2275;
    let t34876 = t5606 * t2279;
    let t34878 = t3785 * t8268;
    (t34861, t34863, t34864, t34866, t34867, t34869, t34870, t34872, t34874, t34876, t34878)
}
