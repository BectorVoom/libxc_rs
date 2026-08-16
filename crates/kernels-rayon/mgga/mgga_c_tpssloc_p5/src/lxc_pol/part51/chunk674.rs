//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 674/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk674(t1874: f64, t2314: f64, t4034: f64, t1266: f64, t1873: f64) -> (f64, f64, f64) {
    let t6522 = 2.0_f64 * t2314 * t1874;
    let t6524 = 2.0_f64 * t4034 * t1874;
    let t6525 = t1266 * t1873;
    (t6522, t6524, t6525)
}
