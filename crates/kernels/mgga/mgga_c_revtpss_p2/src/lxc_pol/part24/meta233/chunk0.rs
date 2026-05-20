//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 991/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk991<F: Float>(t1514: F, t2289: F, t1857: F, t3857: F, t2516: F, t5571: F, t1320: F, t5569: F, t2626: F, t1856: F, t2608: F, t512: F) -> (F, F, F, F, F, F, F) {
    let t13448 = t2289 * t1514;
    let t13584 = t3857 * t1857;
    let t13611 = t5571 * t2516;
    let t13621 = t1320 * t5569;
    let t13630 = t5571 * t2626;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    (t13448, t13584, t13611, t13621, t13630, t13632, t13633)
}
