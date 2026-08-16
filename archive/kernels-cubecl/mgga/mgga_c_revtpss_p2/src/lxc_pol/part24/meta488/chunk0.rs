//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1481/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481<F: Float>(t22212: F, t2516: F, t6922: F, t9593: F, t22185: F, t2619: F, t22404: F, t3920: F, t13725: F, t1904: F, t2439: F, t22446: F, t2435: F) -> (F, F, F, F, F, F) {
    let t73481 = t22212 * t2516;
    let t73499 = t6922 * t9593;
    let t73515 = t22185 * t2619;
    let t73587 = t22404 * t3920;
    let t73593 = t2439 * t13725 * t1904;
    let t73623 = t2435 * t22446;
    (t73481, t73499, t73515, t73587, t73593, t73623)
}
