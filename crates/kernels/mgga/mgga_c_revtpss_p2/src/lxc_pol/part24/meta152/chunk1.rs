//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 771/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk771<F: Float>(t6157: F, t954: F, t2950: F, t2957: F, t4571: F, t4620: F, t6094: F, t6098: F, t6102: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F) -> (F, F) {
    let t6158 = t6157 * t954;
    let t6173 = -F::new(0.17648625e1) * t6114 + F::new(0.3529725e1) * t6121 + t2950 + F::cast_from(0.34431666666666666666e0_f64) * t4571 - F::cast_from(0.34431666666666666667e0_f64) * t6094 + F::new(0.103295e1) * t6098 - F::new(0.516475e0) * t6102 + F::new(0.31558125e0) * t6127 + F::new(0.6311625e0) * t6129 + t2957 + F::cast_from(0.13892666666666666667e0_f64) * t4620 - F::cast_from(0.34731666666666666667e-1_f64) * t6133 + F::new(0.20839e0) * t6136 - F::new(0.104195e0) * t6139;
    (t6158, t6173)
}
