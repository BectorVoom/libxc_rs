//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 999/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk999<F: Float>(t11602: F, t11652: F, t11657: F, t11665: F, t11668: F, t11672: F, t19400: F, t19422: F, t19423: F, t19425: F, t19431: F, t19432: F, t19433: F, t19434: F, t19435: F, t19436: F, t19437: F, t19441: F) -> (F,) {
    let t19912 = -t19400 - t11602 - t11652 - t19422 + t11657 + t19423 + t19425 + t11665 + t11668 - t11672 + t19431 + t19432 + t19433 + t19434 + t19435 + t19436 - t19437 - t19441;
    (t19912,)
}
