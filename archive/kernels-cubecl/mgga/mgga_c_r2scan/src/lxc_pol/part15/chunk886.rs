//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 886/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk886<F: Float>(t6086: F, t8081: F, t6085: F, t7619: F, t6093: F, t1567: F, t2115: F, t494: F, t7338: F) -> (F, F, F, F) {
    let t8082 = t6086 * t8081;
    let t8084 = F::cast_from(0.11643651550782197811e-1_f64) * t6085 * t8082;
    let t8085 = t6086 * t7619;
    let t8086 = t6093 * t8085;
    let t8088 = t2115 * t1567;
    let t8089 = t7338 * t494;
    (t8084, t8086, t8088, t8089)
}
