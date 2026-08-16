//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1910/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1910(t98146: f64, t98152: f64, t98156: f64, t98168: f64, t98180: f64, t98185: f64, t98187: f64, t98193: f64, t98202: f64, t98206: f64, t98222: f64, t98226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t102488 = 0.32012600194825403606e-1_f64 * t98146;
    let t102490 = 0.11433071498151929859e-2_f64 * t98152;
    let t102492 = 0.4065600224742826258e-3_f64 * t98156;
    let t102499 = 7.0_f64 / 12.0_f64 * t98168;
    let t102505 = 0.10164000561857065645e-3_f64 * t98180;
    let t102508 = 0.4065600224742826258e-3_f64 * t98185;
    let t102509 = 0.10164000561857065645e-3_f64 * t98187;
    let t102512 = 0.32012600194825403606e-1_f64 * t98193;
    let t102516 = 0.4065600224742826258e-3_f64 * t98202;
    let t102518 = 0.2032800112371413129e-2_f64 * t98206;
    let t102528 = 0.16006300097412701803e0_f64 * t98222;
    let t102530 = 0.80031500487063509014e-2_f64 * t98226;
    (t102488, t102490, t102492, t102499, t102505, t102508, t102509, t102512, t102516, t102518, t102528, t102530)
}
