//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 901/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk901(t2039: f64, t6862: f64, t31531: f64, t31532: f64, t31539: f64, t31542: f64, t31544: f64, t31548: f64, t31671: f64, t31722: f64, t4034: f64, t574: f64, t6517: f64, t652: f64, t672: f64, t7057: f64, t7061: f64, t7171: f64, t8329: f64, t8450: f64, t8529: f64) -> (f64, f64) {
    let t31726 = t6862 * t2039;
    let t31729 = -2.0_f64 * t31532 * t672 + t31722 * t574 - 2.0_f64 * t31726 * t652 - 2.0_f64 * t4034 * t8529 - 2.0_f64 * t6517 * t7057 - 2.0_f64 * t6517 * t7061 + 3.0_f64 * t7171 * t8450 - t31531 - t31539 - t31542 - t31544 - t31548 + t31671 - t8329;
    (t31726, t31729)
}
