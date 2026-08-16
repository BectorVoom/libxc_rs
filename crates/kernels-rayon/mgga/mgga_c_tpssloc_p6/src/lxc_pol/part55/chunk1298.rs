//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1298/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1298(t125094: f64, t125103: f64, t125121: f64, t125802: f64, t125919: f64, t125939: f64, t125951: f64, t125963: f64, t1858: f64, t8919: f64, t2174: f64, t8110: f64) -> (f64, f64, f64) {
    let t125966 = t125094 + t125103 + t125121 + t125802 + t125919 + t125939 + t125951 + t125963;
    let t125970 = t8919 * t1858;
    let t125975 = t8110 * t2174;
    (t125966, t125970, t125975)
}
