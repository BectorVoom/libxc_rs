//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 897/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk897<F: Float>(t4987: F, t7647: F, t1980: F, t34487: F, t7476: F, t31126: F, t31128: F, t2314: F, t31258: F, t31140: F, t1982: F, t568: F, t31168: F, t13299: F, t31057: F, t35288: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35317 = t7647 * t4987;
    let t35348 = t1980 * t7476 * t34487;
    let t35352 = 0.1324375e0 * t31126;
    let t35353 = 0.57165357490759649296e-3 * t31128;
    let t35359 = t31258 * t2314;
    let t35361 = 0.1528125e-1 * t31140;
    let t35364 = t568 * t1982;
    let t35373 = 0.14291339372689912324e-2 * t31168;
    let t35379 = t31057 * t13299 * t35288;
    (t35317, t35348, t35352, t35353, t35359, t35361, t35364, t35373, t35379)
}
