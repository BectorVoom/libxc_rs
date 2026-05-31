//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1793/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1793<F: Float>(t1012: F, t1222: F, t17401: F, t1803: F, t21017: F, t24699: F, t24706: F, t24736: F, t44959: F, t484: F, t59419: F, t6690: F, t70800: F, t71928: F, t71931: F, t84082: F, t84084: F, t84195: F, t87145: F) -> F {
    let t91398 = -F::cast_from(0.45732285992607719436e-2_f64) * t24699 * t1803 * t484 + F::cast_from(0.13719685797782315831e-1_f64) * t21017 * t24736 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1222 * t1012 * t44959 * t87145 + t71928 / F::cast_from(216.0_f64) + t71931 / F::cast_from(108.0_f64) + F::cast_from(0.57927562257303111285e-1_f64) * t84082 + F::cast_from(0.57165357490759649296e-3_f64) * t84084 - F::cast_from(0.13550306960772657611e-2_f64) * t59419 - F::cast_from(0.25724410870841842184e-2_f64) * t70800 * t6690 - F::cast_from(0.25724410870841842184e-2_f64) * t17401 * t24706 - F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t84195;
    t91398
}
