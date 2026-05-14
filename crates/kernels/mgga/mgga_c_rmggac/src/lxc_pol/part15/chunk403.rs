//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 403/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk403<F: Float>(t4075: F, t945: F, t249: F, t980: F, t1042: F, t388: F, t5: F, t946: F, t973: F, t1090: F, t1101: F, t378: F, t483: F, t7: F, t151: F, t1009: F, t422: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4077 = 0.21687162600603479684e-1 * t945 * t4075;
    let t4078 = t249 * t980;
    let t4080 = 0.32530743900905219526e-1 * t945 * t4078;
    let t4081 = t249 * t1042;
    let t4083 = 0.48159733137676571078e0 * t945 * t4081;
    let t4084 = t388 * t5;
    let t4085 = t4084 * t946;
    let t4087 = t249 * t973;
    let t4089 = 0.16265371950452609763e-1 * t945 * t4087;
    let t4101 = 6.0 * t1090 * t378 * t1101;
    let t4103 = t7 * t483;
    let t4106 = 0.34450798614814814813e-2 * t5 * t4103 * t151;
    let t4111 = 60.0 * t1009 * t422;
    (t4077, t4080, t4083, t4085, t4089, t4101, t4103, t4106, t4111)
}
