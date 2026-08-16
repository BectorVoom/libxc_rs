//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1859/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1859(t23133: f64, t4261: f64, t25111: f64, t81782: f64, t81783: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64, t849: f64, t25068: f64, t2707: f64) -> (f64, f64, f64, f64, f64) {
    let t87332 = t23133 * t4261;
    let t87335 = t81782 * t81783 * t25111;
    let t87338 = t87229 * t81783 * t25115;
    let t87340 = t4166 * t23132;
    let t87341 = t87340 * t849;
    let t87343 = t25068 * t2707;
    (t87332, t87335, t87338, t87341, t87343)
}
