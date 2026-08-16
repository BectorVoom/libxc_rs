//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1450/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1450(t52: f64, t12606: f64, t12874: f64, t12877: f64, t2244: f64, t2250: f64, t4087: f64, t607: f64, t76: f64, t12873: f64, t157: f64, t182: f64, t145: f64, zeta_threshold: f64) -> (f64, f64) {
    let t150 = t52 <= zeta_threshold;
    let t12885 = piecewise3(t150, 0.0_f64, 8.0_f64 / 27.0_f64 * t12874 * t2244 + 8.0_f64 / 9.0_f64 * t12877 * t607 + 4.0_f64 / 9.0_f64 * t4087 * t2250 - 4.0_f64 / 3.0_f64 * t76 * t12606);
    let t12886 = t12873 + t12885;
    let t12887 = t12886 * t157;
    let t12889 = 0.19751673498613801407e-1_f64 * t12887 * t182;
    let t12890 = t145 * t12886;
    (t12889, t12890)
}
