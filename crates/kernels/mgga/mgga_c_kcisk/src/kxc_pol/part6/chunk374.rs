//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 374/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk374<F: Float>(t1783: F, t2464: F, t1310: F, t1771: F, t1773: F, t2449: F, t2456: F, t2460: F, t664: F, t1791: F, t1801: F, t2364: F, t1800: F, t1799: F, t2063: F, t682: F) -> (F, F, F, F, F, F, F, F) {
    let t2465 = t1783 * t2464;
    let t2466 = t1310 * t2465;
    let t2469 = 0.5397236614853195164e-1 * t2449 * t664 - 0.14392630972941853771e0 * t2456 * t664 + t1771 + 0.17990788716177317213e-1 * t1773 * t2460 - 0.5397236614853195164e-1 * t1773 * t2466;
    let t2470 = t2469 * t1791;
    let t2473 = t1801 * t2364;
    let t2474 = t1800 * t2473;
    let t2475 = t1799 * t2474;
    let t2477 = t682 * t2063;
    (t2465, t2466, t2469, t2470, t2473, t2474, t2475, t2477)
}
