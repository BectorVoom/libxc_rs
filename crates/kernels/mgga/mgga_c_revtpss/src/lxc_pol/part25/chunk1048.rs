//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1048/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1048<F: Float>(t12448: F, t12463: F, t1169: F, t1159: F, t3475: F, t426: F, t3478: F, t434: F, t12430: F, t1179: F, t3488: F, t1175: F, t3520: F) -> (F, F, F, F, F) {
    let t12464 = t12448 + t12463;
    let t12465 = t12464 * t1169;
    let t12469 = F::new(1.0) / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = F::new(1.0) / t3478 / t434;
    let t12473 = t12430 * t12472;
    let t12476 = t3488 * t1179;
    let t12481 = t1175 * t3520;
    (t12465, t12470, t12473, t12476, t12481)
}
