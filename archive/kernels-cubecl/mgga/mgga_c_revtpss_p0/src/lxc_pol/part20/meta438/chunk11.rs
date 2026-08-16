//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1662/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1662<F: Float>(t1196: F, t12547: F, t3520: F, t5206: F, t12581: F, t3531: F, t43753: F, t45187: F, t45190: F, t12592: F, t12378: F, t300: F) -> (F, F, F, F, F) {
    let t45310 = F::cast_from(0.69263436422725855036e2_f64) * t1196 * t3520 * t12547 * t5206;
    let t45312 = F::cast_from(0.4155806185363551302e3_f64) * t3531 * t12581;
    let t45316 = F::cast_from(0.91082604192152556044e5_f64) * t1196 * t45187 * t43753 * t45190;
    let t45318 = F::cast_from(0.4101607543286562663e4_f64) * t3531 * t12592;
    let t45319 = t300 * t12378;
    (t45310, t45312, t45316, t45318, t45319)
}
