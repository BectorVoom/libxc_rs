//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 573/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk573(t1801: f64, t4797: f64, t1873: f64, t1869: f64, t3293: f64, t3499: f64, t3500: f64, t8: f64) -> (f64, f64, f64, f64) {
    let t4798 = t1801 * t4797;
    let t4799 = t1873 * t4798;
    let t4800 = t1869 * t4799;
    let t4803 = t3293 * t8 - t3499 + t3500;
    (t4798, t4799, t4800, t4803)
}
