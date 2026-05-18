//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1226/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1226<F: Float>(t25320: F, t8208: F, t8210: F, t8193: F, t8206: F, t852: F, t8214: F, t8216: F, t8194: F, t8197: F, t11399: F, t25122: F, t25297: F, t25302: F, t25305: F, t25308: F, t25313: F, t25316: F, t2650: F, t2797: F, t8095: F, t8120: F, t8211: F, t8220: F, t914: F, t930: F, t931: F) -> F {
    let t25322 = t8208 * t25320 * t8210;
    let t25325 = t8206 * t852 * t8193;
    let t25329 = t8214 * t25320 * t8216;
    let t25332 = t8194 * t25320 * t8197;
    let t25334 = -F::new(0.96161391294453420219e0) * t2797 * t8095 + F::new(0.12020173911806677527e0) * t25297 - F::new(0.10818156520626009775e1) * t930 * t914 * t25122 - F::new(0.21495767235568724176e0) * t25302 - F::new(0.52888765211949381121e1) * t25305 * t931 + F::new(0.1133330683113201024e1) * t25308 - F::new(0.20420978873790287968e1) * t8220 * t2650 - F::new(0.3863627328795003491e-1) * t25313 - F::new(0.51515031050600046546e-1) * t25316 - F::new(0.14866778996637164867e4) * t11399 * t8120 + F::new(0.45352564237957702055e6) * t25322 - F::new(0.36282051390366161644e7) * t25325 * t8211 - F::new(0.45352564237957702055e6) * t25329 + F::new(0.75587607063262836759e5) * t25332;
    t25334
}
