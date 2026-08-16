//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1089/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1089(t16439: f64, t1843: f64, t2016: f64, t22656: f64, t22907: f64, t22909: f64, t22921: f64, t22924: f64, t22926: f64, t22928: f64, t22940: f64, t3758: f64, t5215: f64, t5321: f64, t5354: f64, t6958: f64, t6963: f64, t6993: f64, t7729: f64) -> f64 {
    let t26500 = 0.38381794893125283518e-1_f64 * t22907 + 0.19190897446562641759e-1_f64 * t22909 - t6958 * t5354 - t5321 * t6993 + 2.0_f64 * t5215 * t6963 - t22656 * t1843 + 0.82246703342411321824e-2_f64 * t22921 - t16439 * t2016 + t22924 + t22926 - 0.41123351671205660912e-2_f64 * t22928 + 2.0_f64 * t3758 * t7729 - t5215 * t6993 - 0.19190897446562641759e-1_f64 * t22940;
    t26500
}
