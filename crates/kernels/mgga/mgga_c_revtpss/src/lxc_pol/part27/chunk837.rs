//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 837/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk837<F: Float>(t2411: F, t890: F, t2832: F, t892: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t10564: F, t10627: F, t11054: F, t11061: F, t11064: F, t1940: F, t198: F, t207: F, t2394: F, t2403: F, t2404: F, t2430: F, t262: F, t4541: F, t775: F, t9394: F) -> (F,) {
    let t11071 = t890 * t2411;
    let t11075 = t2832 * t892;
    let t11082 = t11054 * t198 * t207 * t892 + 2.0 * t11061 * t11064 * t198 * t207 + 6.0 * t10627 * t198 * t262 - 3.0 * t11071 * t1940 * t2832 + 9.0 * t11075 * t2403 * t775 + 18.0 * t2394 * t2404 * t4541 + 9.0 * t2403 * t2404 * t2430 - t10552 + t10554 + t10557 + t10560 + t10562 + t10564 + t9394;
    (t11082,)
}
