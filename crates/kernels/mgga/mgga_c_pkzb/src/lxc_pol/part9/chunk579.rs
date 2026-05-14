//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 579/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk579<F: Float>(t133: F, t2387: F, t945: F, t2393: F, t410: F, t2126: F, t394: F, t2434: F, t2421: F, t2433: F, t2436: F, t2439: F, t397: F, t943: F, t946: F) -> (F, F, F, F) {
    let t2442 = t2387 * t133;
    let t2443 = t2442 * t945;
    let t2446 = t2393 * t410;
    let t2447 = t2126 * t394;
    let t2448 = t2434 * t2447;
    let t2453 = 0.13170898365871023197e1 * t2433 * t2436 + 0.13170898365871023197e1 * t2439 * t946 + 0.65854491829355115987e0 * t943 * t2443 - 0.65854491829355115987e0 * t2446 * t2448 + 0.65854491829355115987e0 * t397 * t2421;
    (t2442, t2443, t2448, t2453)
}
