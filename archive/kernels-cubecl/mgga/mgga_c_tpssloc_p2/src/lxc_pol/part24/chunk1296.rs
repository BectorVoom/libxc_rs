//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1296/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1296<F: Float>(t22480: F, t4034: F, t22574: F, t55246: F, t8643: F, t23858: F, t6876: F, t12492: F, t12507: F, t1266: F, t1980: F, t22600: F, t2364: F, t26103: F, t6517: F, t80609: F, t80611: F, t80614: F, t80617: F, t80620: F, t80622: F, t80625: F, t80627: F, t80629: F, t80633: F, t80635: F, t80637: F, t81410: F) -> F {
    let t81412 = F::cast_from(6.0_f64) * t4034 * t22480;
    let t81419 = F::cast_from(9.0_f64) * t22574 * t8643 * t55246;
    let t81422 = F::cast_from(6.0_f64) * t6876 * t23858;
    let t81423 = t12492 * t1980 - F::cast_from(6.0_f64) * t12507 * t6517 - F::cast_from(6.0_f64) * t1266 * t22600 - F::cast_from(6.0_f64) * t2364 * t26103 + t80609 - t80611 + t80614 - t80617 - t80620 - t80622 - t80625 - t80627 - t80629 + t80633 + t80635 + t80637 + t81410 - t81412 - t81419 + t81422;
    t81423
}
