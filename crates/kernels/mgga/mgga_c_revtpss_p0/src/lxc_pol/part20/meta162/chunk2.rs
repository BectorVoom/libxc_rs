//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 874/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874<F: Float>(t213: F, t5744: F, t4086: F, t640: F, t76: F, t159: F, t793: F, t1448: F, t4147: F, t587: F, t65: F) -> (F, F, F, F, F, F) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t6977 = t76 * t640;
    let t7021 = t793 * t159;
    let t7315 = t4147 * t1448;
    let t8779 = F::cast_from(1.0_f64) / t65 / t587;
    (t5745, t5755, t6977, t7021, t7315, t8779)
}
