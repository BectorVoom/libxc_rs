//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3753/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3753<F: Float>(t17445: F, t5373: F, t12866: F, t20933: F, t56756: F, t17789: F, t21017: F, t3601: F, t6573: F, t12916: F, t17747: F, t20962: F) -> (F, F, F, F, F) {
    let t71460 = t5373 * t17445;
    let t71470 = t12866 * t56756 * t20933;
    let t71476 = t21017 * t17789;
    let t71480 = t6573 * t3601;
    let t71490 = t17747 * t12916 * t20962;
    (t71460, t71470, t71476, t71480, t71490)
}
