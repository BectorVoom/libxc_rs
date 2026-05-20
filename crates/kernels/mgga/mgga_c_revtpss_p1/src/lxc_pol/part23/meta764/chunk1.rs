//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2561/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2561<F: Float>(t19607: F, t994: F, t12166: F, t1647: F, t4746: F, t4980: F, t342: F, t379: F, t2435: F, t5048: F) -> (F, F, F, F, F) {
    let t55991 = t994 * t19607;
    let t56017 = t1647 * t12166;
    let t56049 = t4746 * t4980;
    let t56087 = t342 * t379;
    let t56176 = t2435 * t5048;
    (t55991, t56017, t56049, t56087, t56176)
}
