//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3888/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3888<F: Float>(t22294: F, t48823: F, t9816: F, t1398: F, t6843: F, t22245: F, t808: F, t9736: F, t22236: F, t6884: F, t9741: F, t13789: F, t3934: F, t3938: F, t47337: F, t47338: F, t49126: F, t49128: F, t49134: F, t49139: F, t49144: F) -> (F, F) {
    let t74698 = t9816 * t48823 * t22294;
    let t74700 = t6843 * t1398;
    let t74711 = t9736 * t808 * t22245;
    let t74714 = t9736 * t808 * t22236;
    let t74717 = t9741 * t6884;
    let t74719 = -F::cast_from(0.10164000561857065645e-2_f64) * t74698 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t13789 * t74700 * t3938 - F::new(35.0) / F::new(54.0) * t49126 + F::new(7.0) / F::new(6.0) * t49128 + F::new(7.0) / F::new(72.0) * t49134 + F::cast_from(0.22866142996303859718e-3_f64) * t49139 + F::cast_from(0.14291339372689912324e-4_f64) * t49144 + F::cast_from(0.10164000561857065645e-4_f64) * t74711 - F::cast_from(0.50820002809285328225e-4_f64) * t74714 + t47337 - F::new(35.0) / F::new(216.0) * t47338 - F::new(35.0) / F::new(216.0) * t74717;
    (t74700, t74719)
}
