//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1725/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1725<F: Float>(t25231: F, t25242: F, t25253: F, t25275: F, t25283: F, t25251: F, t25256: F, t25258: F, t25263: F, t25267: F, t25271: F, t25278: F, t25280: F) -> (F, F, F, F, F, F) {
    let t26454 = F::cast_from(0.54208002996571016773e-3_f64) * t25231;
    let t26457 = F::cast_from(0.18071592998981862717e-4_f64) * t25242;
    let t26462 = F::cast_from(0.30488190661738479625e-3_f64) * t25253;
    let t26468 = F::new(35.0) / F::new(216.0) * t25275;
    let t26471 = F::cast_from(0.10164000561857065645e-4_f64) * t25283;
    let t26472 = -F::cast_from(0.85748036236139473944e-3_f64) * t25251 + t26462 + F::cast_from(0.22866142996303859718e-3_f64) * t25256 - F::cast_from(0.85748036236139473944e-3_f64) * t25258 + F::cast_from(0.17149607247227894789e-2_f64) * t25263 + F::cast_from(0.80031500487063509014e-2_f64) * t25267 + F::cast_from(0.68598428988911579156e-2_f64) * t25271 + t26468 + F::new(7.0) / F::new(36.0) * t25278 - t25280 / F::new(24.0) - t26471;
    (t26454, t26457, t26462, t26468, t26471, t26472)
}
