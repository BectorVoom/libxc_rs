//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 815/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk815<F: Float>(t10029: F, t10035: F, t10186: F, t11296: F, t11300: F, t12384: F, t12390: F, t12395: F, t12399: F, t12425: F, t12433: F, t125: F, t12895: F, t12899: F, t12929: F, t12990: F, t13055: F, t13057: F, t13067: F, t143: F, t279: F, t2857: F, t296: F, t2986: F, t2990: F, t3620: F, t3642: F, t3686: F, t475: F, t526: F, t5633: F, t5694: F, t8270: F, t8305: F, t988: F) -> F {
    let t13069 = -F::cast_from(0.16213771438917426213e0_f64) * t10029 - F::cast_from(0.87170224553660758101e-3_f64) * t10035 + t12384 * t296 - F::cast_from(2.0_f64) * t3686 * t3620 + F::cast_from(9.0_f64) * t2986 * t10186 + F::cast_from(9.0_f64) * t2986 * t12390 + F::cast_from(9.0_f64) * t11296 * t2990 - F::cast_from(2.0_f64) * t988 * t12395 - t988 * t12399 - F::cast_from(0.35922702030763827282e-1_f64) * t8270 + t12425 + t5633 + (t12433 + t12895) * t125 + F::cast_from(6.0_f64) * t12899 * t143 + F::cast_from(18.0_f64) * t8305 * t11300 - t5694 + F::cast_from(2.0_f64) * t3686 * t3642 + t12990 * t526 + F::cast_from(3.0_f64) * t475 * t143 * t12929 + t13055 * t279 + F::cast_from(18.0_f64) * t2857 * t13057 + t13067;
    t13069
}
