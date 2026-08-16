//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 498/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk498(t236: f64, t3787: f64, t240: f64, t1336: f64, t1351: f64) -> (f64, f64, f64) {
    let t3788 = t3787 * t236;
    let t3789 = t3788 * t240;
    let t3790 = t1336 * t3789;
    let t3791 = t1351 * t1351;
    (t3788, t3790, t3791)
}
