//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 982/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk982<F: Float>(t1468: F, t1940: F, t2071: F, t2403: F, t26425: F, t26590: F, t28460: F, t29599: F, t29602: F, t29606: F, t29713: F, t29716: F, t29719: F, t30: F, t30317: F, t30420: F, t4541: F, t5824: F, t7432: F, t7749: F, t7787: F, t8020: F) -> (F,) {
    let t30438 = 3.0 * t4541 * t30317 + 3.0 * t2403 * t8020 * t7749 - 3.0 * t26425 * t29599 + 3.0 * t2403 * t2071 * t29602 + 3.0 / 2.0 * t2403 * t2071 * t29606 + t1940 * t30420 * t30 / 2.0 - t1940 * t28460 * t7787 + t1940 * t8020 * t1468 + t1940 * t26590 * t29713 - t1940 * t7432 * t29716 - t1940 * t7432 * t29719 / 2.0 + t1940 * t2071 * t5824 / 2.0;
    (t30438,)
}
