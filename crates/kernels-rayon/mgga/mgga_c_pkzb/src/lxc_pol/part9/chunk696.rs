//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 696/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk696(t1245: f64, t914: f64, t1250: f64, t2439: f64, t3246: f64, t3259: f64, t3260: f64, t3266: f64, t3269: f64, t3270: f64, t397: f64, t943: f64, t946: f64) -> (f64, f64) {
    let t3273 = t914 * t1245;
    let t3278 = 0.13170898365871023197e1_f64 * t3259 * t3260 + 0.65854491829355115987e0_f64 * t2439 * t1250 + 0.65854491829355115987e0_f64 * t943 * t3266 - 0.65854491829355115987e0_f64 * t3269 * t3270 + 0.65854491829355115987e0_f64 * t3273 * t946 + 0.65854491829355115987e0_f64 * t397 * t3246;
    (t3273, t3278)
}
