//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 413/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk413(t27: f64, t8532: f64, t2084: f64, t551: f64, t1614: f64, t649: f64, t1652: f64, t674: f64, t8450: f64) -> (f64, f64, f64, f64, f64) {
    let t8533 = t27 * t8532;
    let t8536 = t2084 * t551;
    let t8537 = t27 * t8536;
    let t8561 = t649 * t1614;
    let t8562 = t27 * t8561;
    let t8567 = t649 * t1652;
    let t8568 = t27 * t8567;
    let t8571 = t8450 * t674;
    (t8533, t8537, t8562, t8568, t8571)
}
