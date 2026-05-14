//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 892/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk892<F: Float>(t35096: F, t1181: F, t21118: F, t7351: F, t7426: F, t1165: F, t21955: F, t30806: F, t604: F, t1164: F, t8853: F, t31142: F, t8884: F, t2019: F, t8887: F, t8889: F) -> (F, F, F, F, F, F) {
    let t35097 = 0.21437009059034868486e-2 * t35096;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35101 = 0.12862205435420921092e-2 * t35100;
    let t35113 = t30806 * t1165 * t604 * t21955;
    let t35114 = 0.94344276868812456204e-2 * t35113;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35146 = 7.0 / 72.0 * t35145;
    let t35148 = t2019 * t8887 * t8889;
    (t35097, t35101, t35114, t35137, t35146, t35148)
}
