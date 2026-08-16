//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 854/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk854(t45214: f64, t2676: f64, t36782: f64, t2679: f64, t3621: f64, t9796: f64, t1029: f64, t10827: f64, t13657: f64, t4614: f64, t833: f64, t11784: f64, t2617: f64, t7810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45215 = 0.19171462976960374838e1_f64 * t45214;
    let t45217 = 0.27805936629216998521e0_f64 * t36782 * t2676;
    let t45219 = t9796 * t3621 * t2679;
    let t45222 = t9796 * t1029 * t10827;
    let t45226 = 0.58281247449959539508e2_f64 * t833 * t4614 * t13657;
    let t45228 = t7810 * t11784 * t2617;
    (t45215, t45217, t45219, t45222, t45226, t45228)
}
