//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1912/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1912<F: Float>(t10073: F, t25937: F, t7282: F, t8085: F, t102235: F, t25904: F, t102215: F, t25878: F, t102385: F, t94383: F, t102394: F, t26260: F, t27836: F) -> (F, F, F, F, F, F) {
    let t102610 = t10073 * t7282 * t25937 * t8085;
    let t102615 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t102235;
    let t102617 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t102215;
    let t102629 = t94383 * t102385;
    let t102634 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t102394;
    let t102636 = t10073 * t27836 * t26260;
    (t102610, t102615, t102617, t102629, t102634, t102636)
}
