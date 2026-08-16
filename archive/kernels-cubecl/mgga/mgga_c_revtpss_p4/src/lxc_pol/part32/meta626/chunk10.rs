//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2000/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2000<F: Float>(t101788: F, t7706: F, t29538: F, t7349: F, t101883: F, t101885: F, t108765: F, t108816: F, t2048: F, t28112: F, t28116: F, t28119: F, t28635: F, t29554: F, t7352: F, t7709: F, t7964: F, t95294: F) -> F {
    let t110008 = t101788 * t7706;
    let t110010 = t29538 * t7349;
    let t110012 = t101883 + t101885 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t95294 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108765 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108816 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29554 * t7352 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28112 * t7964 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28116 * t7964 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28119 * t7964 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7709 * t28635 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t110008 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t110010;
    t110012
}
