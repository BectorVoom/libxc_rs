//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1612/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1612<F: Float>(t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F) -> (F, F) {
    let t15094 = F::cast_from(0.27785333333333333334e0_f64) * t14781;
    let t15115 = -F::cast_from(0.3529725e1_f64) * t14809 - F::cast_from(0.17648625e1_f64) * t14811 + F::cast_from(0.6311625e0_f64) * t14814 + F::cast_from(0.31558125e0_f64) * t14816 + F::cast_from(0.46308888888888888889e-1_f64) * t14818 + F::cast_from(0.45908888888888888888e0_f64) * t11137 + F::cast_from(0.11477222222222222222e0_f64) * t11139 - F::cast_from(0.34431666666666666666e0_f64) * t11141 - F::cast_from(0.17215833333333333333e0_f64) * t11143 + F::cast_from(0.6311625e0_f64) * t14824 + F::cast_from(0.57386111111111111112e0_f64) * t14728;
    (t15094, t15115)
}
