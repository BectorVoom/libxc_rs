//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1839/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1839(t24833: f64, t7377: f64, t2144: f64, t3507: f64, t3625: f64, t1215: f64, t7348: f64, t1246: f64, t1170: f64, t7381: f64, t2121: f64, t210: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24834 = t24833 * t7377;
    let t24837 = t2144 * t3507;
    let t24838 = t24837 * t3625;
    let t24840 = t7348 * t1215;
    let t24841 = t24840 * t1246;
    let t24844 = t1170 * t7381;
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    (t24834, t24837, t24838, t24841, t24844, t24845, t24847)
}
