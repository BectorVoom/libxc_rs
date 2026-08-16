//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2867/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2867(t291: f64, t59846: f64, t59860: f64, t59873: f64, t59887: f64, t17297: f64, t2932: f64, t2860: f64, t5737: f64, t10756: f64, t10771: f64, t10825: f64, t13716: f64, t14263: f64, t14337: f64, t14366: f64, t14370: f64, t14425: f64, t14453: f64, t14456: f64, t1581: f64, t17366: f64, t17496: f64, t17500: f64, t2863: f64, t2880: f64, t2905: f64, t2906: f64, t2924: f64, t2930: f64, t311: f64, t41821: f64, t49099: f64, t49104: f64, t49422: f64, t5762: f64, t5775: f64, t5794: f64, t59637: f64, t59774: f64, t59788: f64, t59802: f64, t59815: f64, t59829: f64, t950: f64) -> (f64, f64) {
    let t59891 = 0.621814e-1_f64 * (t59846 + t59860 + t59873 + t59887) * t291;
    let t59895 = t17297 * t2932;
    let t59920 = t5737 * t2860;
    let t59928 = t59637 - 0.19751673498613801407e-1_f64 * t59774 - 0.310907e-1_f64 * (t59788 + t59802 + t59815 + t59829) * t311 + t59891 - 0.23392894490538584828e1_f64 * t2905 * t17366 * t950 + 0.34631718211362927518e2_f64 * t2930 * t59895 * t950 + 0.69263436422725855036e2_f64 * t10825 * t17496 + 0.20508037716432813316e4_f64 * t41821 * t17500 - 0.23392894490538584828e1_f64 * t14263 * t14453 - 0.2077903092681775651e3_f64 * t49099 * t14456 + 0.34631718211362927517e2_f64 * t14337 * t14366 + 0.20508037716432813315e4_f64 * t49104 * t14370 + 0.35089341735807877242e1_f64 * t2930 * t5775 * t2924 + 0.6233709278045326953e3_f64 * t10756 * t5794 * t2906 - 0.23392894490538584828e1_f64 * t2905 * t1581 * t13716 - 2.0_f64 * t59920 * t2863 + 24.0_f64 * t49422 * t14425 - 0.19298375398431042081e3_f64 * t10771 * t5762 * t2880;
    (t59891, t59928)
}
