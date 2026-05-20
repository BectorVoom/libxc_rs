//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1893/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1893<F: Float>(t1224: F, t65: F, t5052: F, t1266: F, t1808: F, t26821: F, t26822: F, t26832: F, t26836: F, t26852: F, t26867: F, t29031: F, t29034: F, t29037: F, t29040: F, t29047: F, t5386: F, t5407: F) -> (F, F, F) {
    let t29048 = t65 * t1224;
    let t29049 = t29048 * t5052;
    let t29052 = -t26821 + F::cast_from(0.28582678745379824648e-3_f64) * t26822 - t29031 / F::new(864.0) - F::cast_from(0.28582678745379824648e-3_f64) * t26832 - F::cast_from(0.19055119163586549765e-3_f64) * t29034 - t26836 / F::new(864.0) - F::cast_from(0.28582678745379824648e-3_f64) * t29037 * t1266 + F::cast_from(0.85748036236139473944e-3_f64) * t29040 * t5386 - F::cast_from(0.28582678745379824648e-3_f64) * t26852 * t1808 - F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t5407 - t29047 * t29049 / F::new(144.0);
    (t29048, t29049, t29052)
}
