//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 666/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk666(t856: f64, t68: f64, t261: f64, t2751: f64) -> (f64, f64, f64, f64) {
    let t10108 = t856 * t856;
    let t10109 = 1.0_f64 / t10108;
    let t10110 = t68 * t10109;
    let t10143 = 1.0_f64 / t2751 / t261;
    (t10108, t10109, t10110, t10143)
}
