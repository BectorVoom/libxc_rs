//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 815/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk815(t10029: f64, t10035: f64, t10186: f64, t11296: f64, t11300: f64, t12384: f64, t12390: f64, t12395: f64, t12399: f64, t12425: f64, t12433: f64, t125: f64, t12895: f64, t12899: f64, t12929: f64, t12990: f64, t13055: f64, t13057: f64, t13067: f64, t143: f64, t279: f64, t2857: f64, t296: f64, t2986: f64, t2990: f64, t3620: f64, t3642: f64, t3686: f64, t475: f64, t526: f64, t5633: f64, t5694: f64, t8270: f64, t8305: f64, t988: f64) -> f64 {
    let t13069 = -0.16213771438917426213e0_f64 * t10029 - 0.87170224553660758101e-3_f64 * t10035 + t12384 * t296 - 2.0_f64 * t3686 * t3620 + 9.0_f64 * t2986 * t10186 + 9.0_f64 * t2986 * t12390 + 9.0_f64 * t11296 * t2990 - 2.0_f64 * t988 * t12395 - t988 * t12399 - 0.35922702030763827282e-1_f64 * t8270 + t12425 + t5633 + (t12433 + t12895) * t125 + 6.0_f64 * t12899 * t143 + 18.0_f64 * t8305 * t11300 - t5694 + 2.0_f64 * t3686 * t3642 + t12990 * t526 + 3.0_f64 * t475 * t143 * t12929 + t13055 * t279 + 18.0_f64 * t2857 * t13057 + t13067;
    t13069
}
