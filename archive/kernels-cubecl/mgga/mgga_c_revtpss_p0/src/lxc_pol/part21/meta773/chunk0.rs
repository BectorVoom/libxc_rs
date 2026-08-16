//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2745/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745<F: Float>(t50504: F, t1558: F, t2722: F, t10726: F, t2661: F, t2724: F, t4416: F, t4352: F, t10722: F, t4435: F, t14751: F, t2652: F) -> (F, F, F, F, F, F) {
    let t50505 = F::cast_from(0.22866142996303859718e-3_f64) * t50504;
    let t50511 = t1558 * t2722;
    let t50518 = t2661 * t10726 * t4416 * t2724;
    let t50522 = t2661 * t10726 * t4352 * t2724;
    let t50524 = t10722 * t4435;
    let t50526 = t2652 * t14751;
    (t50505, t50511, t50518, t50522, t50524, t50526)
}
