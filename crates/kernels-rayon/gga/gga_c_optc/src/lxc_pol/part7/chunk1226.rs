//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1226/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1226(t25320: f64, t8208: f64, t8210: f64, t8193: f64, t8206: f64, t852: f64, t8214: f64, t8216: f64, t8194: f64, t8197: f64, t11399: f64, t25122: f64, t25297: f64, t25302: f64, t25305: f64, t25308: f64, t25313: f64, t25316: f64, t2650: f64, t2797: f64, t8095: f64, t8120: f64, t8211: f64, t8220: f64, t914: f64, t930: f64, t931: f64) -> f64 {
    let t25322 = t8208 * t25320 * t8210;
    let t25325 = t8206 * t852 * t8193;
    let t25329 = t8214 * t25320 * t8216;
    let t25332 = t8194 * t25320 * t8197;
    let t25334 = -0.96161391294453420219e0_f64 * t2797 * t8095 + 0.12020173911806677527e0_f64 * t25297 - 0.10818156520626009775e1_f64 * t930 * t914 * t25122 - 0.21495767235568724176e0_f64 * t25302 - 0.52888765211949381121e1_f64 * t25305 * t931 + 0.1133330683113201024e1_f64 * t25308 - 0.20420978873790287968e1_f64 * t8220 * t2650 - 0.3863627328795003491e-1_f64 * t25313 - 0.51515031050600046546e-1_f64 * t25316 - 0.14866778996637164867e4_f64 * t11399 * t8120 + 0.45352564237957702055e6_f64 * t25322 - 0.36282051390366161644e7_f64 * t25325 * t8211 - 0.45352564237957702055e6_f64 * t25329 + 0.75587607063262836759e5_f64 * t25332;
    t25334
}
