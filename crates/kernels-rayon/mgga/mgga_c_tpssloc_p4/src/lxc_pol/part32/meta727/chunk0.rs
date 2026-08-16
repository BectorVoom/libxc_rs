//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2353/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2353(t5: f64, t104758: f64, t104783: f64, t104813: f64, t104858: f64, t104885: f64, t104916: f64, t104942: f64, t104971: f64, t112: f64, t671: f64, t7982: f64, t111: f64, t29485: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t104975 = piecewise3(t8, 0.0_f64, t104758 + t104783 + t104813 + t104858 + t104885 + t104916 + t104942 + t104971);
    let t104976 = t104975 * t112;
    let t104977 = t7982 * t671;
    let t104990 = t29485 * t111;
    (t104976, t104977, t104990)
}
