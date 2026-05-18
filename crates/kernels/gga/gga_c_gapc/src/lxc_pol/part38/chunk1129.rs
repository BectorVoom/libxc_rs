//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1129/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1129<F: Float>(t1038: F, t28622: F, t311: F, t4043: F, t6851: F, t1026: F, t1093: F, t2153: F, t11417: F, t11971: F, t761: F, t1645: F, t189: F) -> (F, F, F, F) {
    let t34013 = t311 * t6851 * t4043 * M_PI * t1038 * t28622;
    let t34016 = t2153 * t1026 * t1093;
    let t34019 = t761 * t11417 * t11971;
    let t34021 = t189 * t1645;
    (t34013, t34016, t34019, t34021)
}
