//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1676/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1676<F: Float>(t88382: F, t88396: F, t88412: F, t88427: F, t915: F, t935: F, t1609: F, t23547: F, t2874: F, t2924: F, t78329: F, t11385: F, t19255: F, t6141: F) -> (F, F, F, F) {
    let t88432 = F::new(1.0) * t915 * (t88382 + t88396 + t88412 + t88427) * t935;
    let t88445 = F::new(8.0) * t2874 * t23547 * t1609;
    let t88448 = F::cast_from(0.64327917994770140268e2_f64) * t2924 * t78329 * t1609;
    let t88451 = F::cast_from(0.3103560775156404018e4_f64) * t11385 * t19255 * t6141;
    (t88432, t88445, t88448, t88451)
}
