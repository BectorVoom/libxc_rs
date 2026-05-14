//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1060/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1060<F: Float>(t2271: F, t4182: F, t14591: F, t3784: F, t6370: F, t19005: F, t4204: F, t4203: F, t1445: F, t4208: F, t6333: F, t1413: F, t6308: F, t1489: F, t14320: F, t469: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t21278 = t2271 * t4182;
    let t21280 = t3784 * t14591;
    let t21281 = t21280 * t6370;
    let t21283 = t4204 * t19005;
    let t21284 = t4203 * t21283;
    let t21286 = t4208 * t1445;
    let t21287 = t21286 * t6333;
    let t21289 = t6308 * t1413;
    let t21290 = t21289 * sigma0;
    let t21291 = t21290 * t1489;
    let t21293 = t14320 * t469;
    (t21278, t21281, t21283, t21284, t21287, t21289, t21291, t21293)
}
