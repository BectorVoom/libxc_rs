//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 968/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk968<F: Float>(t11024: F, t10983: F, t10988: F, t10991: F, t10996: F, t11001: F, t11006: F, t11008: F, t11014: F, t11018: F, t11022: F, t10614: F, t10618: F, t10621: F, t10625: F, t10629: F, t10633: F, t10637: F, t10643: F, t10653: F, t10657: F, t10925: F, t10975: F) -> (F, F) {
    let t11025 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11024;
    let t11026 = -t10983 - t10988 + t10991 + t10996 - t11001 + t11006 - F::cast_from(0.81300399444200075504e-3_f64) * t11008 + t11014 - t11018 - t11022 - t11025;
    let t11028 = -t10614 + F::cast_from(0.15243824895787514157e-3_f64) * t10643 + t10618 - t10621 + t10625 - t10629 - t10633 + F::cast_from(0.72042316457491791906e-3_f64) * t10653 + t10637 - t10657 + t10925 + t10975 + t11026;
    (t11025, t11028)
}
