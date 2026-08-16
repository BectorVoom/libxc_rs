//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2691/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691(t20670: f64, t225: f64, t1834: f64, t6414: f64, t6387: f64, t20553: f64, t562: f64, t20489: f64, t16036: f64, t16047: f64, t16055: f64, t1825: f64, t19654: f64, t19661: f64, t19735: f64, t19743: f64, t19744: f64, t19810: f64, t20018: f64, t20473: f64, t20638: f64, t5250: f64, t5287: f64, t5333: f64, t5334: f64, t5336: f64, t5344: f64, t54963: f64, t57704: f64, t6378: f64, t74599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74930 = t20670 * t225;
    let t74937 = t1834 * t6414;
    let t74941 = t1834 * t6387;
    let t74949 = t562 * t20553;
    let t74967 = t562 * t20489;
    let t74996 = 6.0_f64 * t16036 * t20473 * t5334 - 36.0_f64 * t16047 * t19744 * t74967 - 3.0_f64 * t1825 * t5344 * t57704 + 18.0_f64 * t19735 * t19743 * t5334 - 3.0_f64 * t19743 * t5287 * t5344 + 14.0_f64 * t5250 * t5334 * t74967 + 6.0_f64 * t5333 * t5336 * t6378 + 24.0_f64 * t54963 * t74599 * t74967 + 6.0_f64 * t16055 * t20638 + 6.0_f64 * t19654 * t19661 - 6.0_f64 * t19810 * t20018;
    (t74930, t74937, t74941, t74949, t74967, t74996)
}
