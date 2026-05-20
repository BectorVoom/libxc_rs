//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1370/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1370<F: Float>(t13039: F, t44372: F, t44373: F, t13045: F, t42871: F, t3597: F, t3603: F, t3367: F, t1209: F, t13147: F, t17708: F, t12854: F, t17350: F) -> (F, F, F, F, F, F, F) {
    let t44441 = t44372 * t13039 * t44373;
    let t44442 = t42871 * t13045;
    let t44448 = t44372 * t3597 * t44373;
    let t44449 = t42871 * t3603;
    let t44458 = t3603 * t3367;
    let t44500 = t1209 * t13147 * t17708;
    let t44510 = t12854 * t17350;
    (t44441, t44442, t44448, t44449, t44458, t44500, t44510)
}
