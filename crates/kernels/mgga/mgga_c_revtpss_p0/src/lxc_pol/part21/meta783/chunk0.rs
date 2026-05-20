//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2809/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2809<F: Float>(t10535: F, t136: F, t2457: F, t4424: F, t10523: F, t14568: F, t2482: F, t2801: F, t4423: F, t879: F, t14606: F, t1568: F, t2722: F) -> (F, F, F, F, F) {
    let t51614 = t10535 * t4424 * t136 * t2457;
    let t51615 = F::cast_from(0.34697458558045176417e-2_f64) * t51614;
    let t51617 = t14568 * t10523;
    let t51621 = t2482 * t879 * t4423 * t2801;
    let t51623 = t14606 * t10523;
    let t51625 = t1568 * t2722;
    (t51615, t51617, t51621, t51623, t51625)
}
