//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1166/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1166(t2127: f64, t3545: f64, t3475: f64, t460: f64, t7320: f64, t2132: f64, t607: f64, t2136: f64, t3535: f64, t7338: f64, t461: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24704 = t2127 * t3545 / 432.0_f64;
    let t24705 = t3475 * t460;
    let t24706 = t24705 * t7320;
    let t24711 = t2132 * t607;
    let t24712 = t24711 * t2136;
    let t24716 = t3535 * t7338;
    let t24719 = t52 * t461;
    (t24704, t24705, t24706, t24712, t24716, t24719)
}
