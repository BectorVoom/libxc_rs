//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 387/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk387<F: Float>(t2507: F, t673: F, t716: F, t720: F, t415: F, t1876: F, t1877: F, t2063: F, t1882: F, t2372: F, t706: F, t1887: F, t2487: F) -> (F, F, F, F, F, F, F) {
    let t2508 = t673 * t2507;
    let t2509 = t2508 * t716;
    let t2510 = t2509 * t720;
    let t2511 = t415 * t2510;
    let t2514 = t1876 * t1877 * t2063;
    let t2517 = t1882 * t2372;
    let t2518 = t706 * t2517;
    let t2521 = t1887 * t2487;
    (t2509, t2510, t2511, t2514, t2517, t2518, t2521)
}
