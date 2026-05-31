//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1158/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1158<F: Float>(t26205: F, t6954: F, t45958: F, t7342: F, t25110: F, t26179: F, t26169: F, t6963: F, t45963: F, t2048: F, t25102: F, t25159: F, t26172: F, t26175: F, t26187: F, t6960: F, t7343: F, t7352: F, t92639: F, t92654: F, t92696: F, t92709: F) -> F {
    let t95255 = t6954 * t26205;
    let t95259 = t45958 * t7342;
    let t95268 = t26179 * t25110;
    let t95270 = t6963 * t26169;
    let t95276 = t45963 * t7342;
    let t95281 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t95255 - F::cast_from(2.0_f64) * t92639 * t2048 - F::cast_from(5.0_f64) * t95259 * t6960 - F::cast_from(2.0_f64) * t92709 * t2048 - F::cast_from(10.0_f64) * t26187 * t25110 - F::cast_from(4.0_f64) * t25102 * t7352 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t95268 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t95270 - F::cast_from(5.0_f64) * t7343 * t92654 - F::cast_from(2.0_f64) * t6963 * t26172 + F::cast_from(30.0_f64) * t95276 * t25159 + F::cast_from(30.0_f64) * t26175 * t92696;
    t95281
}
