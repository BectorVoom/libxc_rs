//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1276/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1276(t32441: f64, t4997: f64, t1017: f64, t1207: f64, t1209: f64, t1742: f64, t372: f64, t471: f64, t477: f64, t32440: f64, t5001: f64, t1730: f64, t32447: f64) -> (f64, f64, f64, f64, f64) {
    let t125398 = t32441 * t4997;
    let t125402 = t1207 * t1209 * t1742 * t1017;
    let t125407 = t471 * t477 * t1742 * t372;
    let t125410 = t5001 * t32440;
    let t125413 = t1730 * t32447;
    (t125398, t125402, t125407, t125410, t125413)
}
