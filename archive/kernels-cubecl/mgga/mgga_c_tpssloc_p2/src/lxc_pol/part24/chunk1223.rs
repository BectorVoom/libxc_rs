//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1223/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1223<F: Float>(t16535: F, t1873: F, t6534: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2319: F, t23862: F, t23877: F, t23880: F, t23886: F, t23888: F, t23890: F, t577: F, t7010: F) -> (F, F, F) {
    let t23892 = F::cast_from(27.0_f64) * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = F::cast_from(54.0_f64) * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = F::cast_from(27.0_f64) * t3941 * t23896;
    let t23900 = F::cast_from(0.135e2_f64) * t1401 * t22479;
    let t23901 = F::cast_from(0.45e1_f64) * t23862 * t577 + F::cast_from(27.0_f64) * t23877 * t671 + F::cast_from(27.0_f64) * t23880 * t2319 + F::cast_from(0.135e2_f64) * t7010 * t2363 + t23886 + t23888 + t23890 + t23892 + t23895 + t23898 + t23900;
    (t23893, t23896, t23901)
}
