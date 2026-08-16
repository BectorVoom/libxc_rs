//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2867/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2867<F: Float>(t291: F, t59846: F, t59860: F, t59873: F, t59887: F, t17297: F, t2932: F, t2860: F, t5737: F, t10756: F, t10771: F, t10825: F, t13716: F, t14263: F, t14337: F, t14366: F, t14370: F, t14425: F, t14453: F, t14456: F, t1581: F, t17366: F, t17496: F, t17500: F, t2863: F, t2880: F, t2905: F, t2906: F, t2924: F, t2930: F, t311: F, t41821: F, t49099: F, t49104: F, t49422: F, t5762: F, t5775: F, t5794: F, t59637: F, t59774: F, t59788: F, t59802: F, t59815: F, t59829: F, t950: F) -> (F, F) {
    let t59891 = F::cast_from(0.621814e-1_f64) * (t59846 + t59860 + t59873 + t59887) * t291;
    let t59895 = t17297 * t2932;
    let t59920 = t5737 * t2860;
    let t59928 = t59637 - F::cast_from(0.19751673498613801407e-1_f64) * t59774 - F::cast_from(0.310907e-1_f64) * (t59788 + t59802 + t59815 + t59829) * t311 + t59891 - F::cast_from(0.23392894490538584828e1_f64) * t2905 * t17366 * t950 + F::cast_from(0.34631718211362927518e2_f64) * t2930 * t59895 * t950 + F::cast_from(0.69263436422725855036e2_f64) * t10825 * t17496 + F::cast_from(0.20508037716432813316e4_f64) * t41821 * t17500 - F::cast_from(0.23392894490538584828e1_f64) * t14263 * t14453 - F::cast_from(0.2077903092681775651e3_f64) * t49099 * t14456 + F::cast_from(0.34631718211362927517e2_f64) * t14337 * t14366 + F::cast_from(0.20508037716432813315e4_f64) * t49104 * t14370 + F::cast_from(0.35089341735807877242e1_f64) * t2930 * t5775 * t2924 + F::cast_from(0.6233709278045326953e3_f64) * t10756 * t5794 * t2906 - F::cast_from(0.23392894490538584828e1_f64) * t2905 * t1581 * t13716 - F::cast_from(2.0_f64) * t59920 * t2863 + F::cast_from(24.0_f64) * t49422 * t14425 - F::cast_from(0.19298375398431042081e3_f64) * t10771 * t5762 * t2880;
    (t59891, t59928)
}
