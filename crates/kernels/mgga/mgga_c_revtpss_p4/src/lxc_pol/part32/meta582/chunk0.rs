//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1910/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1910<F: Float>(t98146: F, t98152: F, t98156: F, t98168: F, t98180: F, t98185: F, t98187: F, t98193: F, t98202: F, t98206: F, t98222: F, t98226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t102488 = F::cast_from(0.32012600194825403606e-1_f64) * t98146;
    let t102490 = F::cast_from(0.11433071498151929859e-2_f64) * t98152;
    let t102492 = F::cast_from(0.4065600224742826258e-3_f64) * t98156;
    let t102499 = F::new(7.0) / F::new(12.0) * t98168;
    let t102505 = F::cast_from(0.10164000561857065645e-3_f64) * t98180;
    let t102508 = F::cast_from(0.4065600224742826258e-3_f64) * t98185;
    let t102509 = F::cast_from(0.10164000561857065645e-3_f64) * t98187;
    let t102512 = F::cast_from(0.32012600194825403606e-1_f64) * t98193;
    let t102516 = F::cast_from(0.4065600224742826258e-3_f64) * t98202;
    let t102518 = F::cast_from(0.2032800112371413129e-2_f64) * t98206;
    let t102528 = F::cast_from(0.16006300097412701803e0_f64) * t98222;
    let t102530 = F::cast_from(0.80031500487063509014e-2_f64) * t98226;
    (t102488, t102490, t102492, t102499, t102505, t102508, t102509, t102512, t102516, t102518, t102528, t102530)
}
