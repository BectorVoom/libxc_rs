//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1068/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1068<F: Float>(t19846: F, t19856: F, t14160: F, t14162: F, t19840: F, t19851: F, t19854: F, t19859: F, t19863: F, t19866: F, t19871: F, t19876: F, t19879: F, t19884: F, t19889: F, t19893: F, t19898: F, t19902: F, t19907: F, t19910: F) -> (F,) {
    let t21446 = 0.23214722222222222222e-2 * t19846;
    let t21449 = 0.23214722222222222222e-2 * t19856;
    let t21465 = -0.61905925925925925926e-2 * t19840 - t21446 + 0.34822083333333333332e-2 * t19851 + 0.92858888888888888886e-2 * t19854 + t21449 + 0.46429444444444444444e-2 * t19859 + 0.23214722222222222222e-2 * t19863 + 0.11607361111111111111e-2 * t19866 - 0.77382407407407407406e-3 * t14160 + 0.11349419753086419753e-1 * t14162 - 0.23214722222222222222e-2 * t19871 + 0.69644166666666666664e-2 * t19876 - 0.61905925925925925926e-2 * t19879 + 0.51588271604938271604e-3 * t19884 + 0.23214722222222222222e-2 * t19889 + 0.61905925925925925926e-2 * t19893 + 0.38691203703703703703e-3 * t19898 + 0.20635308641975308642e-2 * t19902 - 0.38691203703703703703e-3 * t19907 + 0.11607361111111111111e-2 * t19910;
    (t21465,)
}
