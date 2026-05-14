//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 878/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk878<F: Float>(t1159: F, t3475: F, t426: F, t3478: F, t434: F, t12430: F, t1179: F, t3488: F, t1175: F, t3520: F, t3519: F, t444: F, t439: F, t1187: F, t3497: F, t3523: F) -> (F, F, F, F, F, F, F, F) {
    let t12469 = 1.0 / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = 1.0 / t3478 / t434;
    let t12473 = t12430 * t12472;
    let t12476 = t3488 * t1179;
    let t12481 = t1175 * t3520;
    let t12485 = 1.0 / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12487 = t3497 * t1187;
    let t12488 = t12487 * t3523;
    (t12470, t12473, t12476, t12481, t12485, t12486, t12487, t12488)
}
