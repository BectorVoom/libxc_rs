//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta203(t25: f64, t514: f64, t28: f64, t517: f64, t1376: f64, t68: f64, t522: f64, t9212: f64, t9214: f64, t3824: f64, t592: f64, t1287: f64, t2221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11985, t11987, t11998, t12000, t12019, t12021, t12044, t12046, t12048, t12052) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk844(t25, t514, t28, t517, t1376, t68, t522, t9212, t9214, t3824, t592, t1287, t2221);
    (t11985, t11987, t11998, t12000, t12019, t12021, t12044, t12046, t12048, t12052)
}
