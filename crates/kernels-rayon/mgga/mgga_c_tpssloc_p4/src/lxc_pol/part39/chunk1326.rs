//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1326/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1326(t2205: f64, t5381: f64, t30395: f64, t576: f64, t2212: f64, t5363: f64, t1395: f64, t8299: f64, t110274: f64, t110276: f64, t110284: f64, t111215: f64, t1852: f64, t3: f64, t30133: f64, t3932: f64, t3946: f64, t5364: f64, t580: f64, t8200: f64, t8217: f64, t8284: f64) -> f64 {
    let t111302 = 2.0_f64 * t2205 * t5381;
    let t111308 = 2.0_f64 * t576 * t30395;
    let t111310 = 2.0_f64 * t5363 * t2212;
    let t111312 = 2.0_f64 * t1395 * t8299;
    let t111314 = t111215 * t3 * t580 + t1852 * t30133 + t3932 * t8299 + t3946 * t8284 + 2.0_f64 * t5364 * t8217 + 2.0_f64 * t5381 * t8200 + t110274 + t110276 + t110284 + t111302 + t111308 + t111310 + t111312;
    t111314
}
