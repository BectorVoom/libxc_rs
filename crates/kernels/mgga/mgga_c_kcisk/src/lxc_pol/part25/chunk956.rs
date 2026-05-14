//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 956/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk956<F: Float>(t16930: F, t7055: F, t6790: F, t682: F, t1648: F, t4629: F, t12760: F, t139: F, t41: F, t7017: F, t1417: F, t7061: F, t11401: F, t2487: F, t4605: F, t4657: F) -> (F, F, F, F, F, F, F) {
    let t16931 = t7055 * t16930;
    let t16934 = t682 * t6790;
    let t16935 = t16934 * t1648;
    let t16936 = t4629 * t16935;
    let t16940 = t139 * t12760 * t41;
    let t16941 = t16940 * t7017;
    let t16945 = 0.13140859333333333333e-2 * t1417 * t7061;
    let t16946 = t11401 * t2487;
    let t16947 = t16946 * t4605;
    let t16950 = t4657 * t2487;
    (t16931, t16935, t16936, t16941, t16945, t16947, t16950)
}
