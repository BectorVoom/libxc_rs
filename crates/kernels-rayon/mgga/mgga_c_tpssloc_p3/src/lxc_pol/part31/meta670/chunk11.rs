//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2000/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000(t2031: f64, t96461: f64, t96469: f64, t22549: f64, t23963: f64, t26009: f64, t26016: f64, t26954: f64, t34125: f64, t84216: f64, t84229: f64, t90101: f64, t90104: f64, t91922: f64, t92040: f64, t92052: f64, t9239: f64, t96418: f64, t96458: f64, t96466: f64) -> f64 {
    let t102163 = t2031 * t96461;
    let t102168 = t2031 * t96469;
    let t102171 = -880.0_f64 / 27.0_f64 * t91922 - 70.0_f64 * t84216 * t96418 - 40.0_f64 * t9239 * t34125 * t26009 + 88.0_f64 / 27.0_f64 * t84229 + 20.0_f64 / 3.0_f64 * t90101 * t26954 + 20.0_f64 / 3.0_f64 * t90104 * t26954 + 20.0_f64 / 3.0_f64 * t26016 * t92040 + 20.0_f64 / 3.0_f64 * t26016 * t92052 + 20.0_f64 * t23963 * t96458 + 20.0_f64 / 3.0_f64 * t22549 * t102163 + 10.0_f64 * t23963 * t96466 + 10.0_f64 / 3.0_f64 * t22549 * t102168;
    t102171
}
