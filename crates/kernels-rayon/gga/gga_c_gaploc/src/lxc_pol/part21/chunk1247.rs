//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1247/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1247(t16455: f64, t32889: f64, t7585: f64, t10820: f64, t22693: f64, t7427: f64, t10930: f64, t5750: f64, t579: f64, t23344: f64, t7573: f64, t2628: f64, t8516: f64) -> (f64, f64, f64, f64, f64) {
    let t33101 = 0.23005755572352449806e2_f64 * t16455 * t7585 * t32889;
    let t33105 = 0.1656414401209376386e3_f64 * t7427 * t22693 * t10820;
    let t33109 = 0.73618417831527839379e2_f64 * t10930 * t579 * t5750 * t10820;
    let t33112 = 0.13803453343411469884e2_f64 * t23344 * t7573 * t32889;
    let t33113 = t8516 * t2628;
    (t33101, t33105, t33109, t33112, t33113)
}
