//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 586/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk586<F: Float>(t678: F, t7944: F, t2153: F, t275: F, t1347: F, t669: F, t1288: F, t668: F, t72: F, t2028: F, t2604: F, t7245: F, t7289: F, t7383: F, t7402: F, t7430: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7945 = t7944 * t678;
    let t7946 = 0.42564599893297839398e-5 * t7945;
    let t7947 = t275 * t2153;
    let t7949 = t1347 * t669;
    let t7950 = t1288 * t668;
    let t7951 = t72 * t7950;
    let t7952 = t2604 * t2028;
    let t7953 = 0.11974241701863808564e0 * t7952;
    let t8026 = 0.39726959900411316772e-4 * t7245;
    let t8040 = 0.10909864661698136692e0 * t7289;
    let t8081 = 0.15965655602485078085e0 * t7383;
    let t8086 = 0.39726959900411316772e-4 * t7402;
    let t8092 = 0.39726959900411316772e-4 * t7430;
    (t7946, t7947, t7949, t7950, t7951, t7953, t8026, t8040, t8081, t8086, t8092)
}
