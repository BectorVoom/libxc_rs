//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2181/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2181(t19844: f64, t3726: f64, t1831: f64, t53906: f64, t16336: f64, t5314: f64, t53880: f64, t19930: f64, t3866: f64, t1351: f64, t6414: f64, t120: f64, t19731: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56738 = t3726 * t19844;
    let t56776 = t53906 * t1831;
    let t56779 = t16336 * t5314;
    let t56795 = t53880 * t1831;
    let t56797 = t3866 * t19930;
    let t56812 = t6414 * t1351;
    let t56817 = t120 * t19731;
    (t56738, t56776, t56779, t56795, t56797, t56812, t56817)
}
