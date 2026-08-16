//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 649/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk649(t6889: f64, t8458: f64, t1985: f64, t1998: f64, t59: f64) -> (f64, f64, f64) {
    let t8459 = t6889 * t8458;
    let t8461 = 0.16449340668482264365e-1_f64 * t1985 * t8459;
    let t8462 = t1998 * t59;
    (t8459, t8461, t8462)
}
