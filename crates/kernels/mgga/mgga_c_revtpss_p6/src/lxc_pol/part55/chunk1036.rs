//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1036/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1036<F: Float>(t1401: F, t32284: F, t1405: F, t8591: F, t1412: F, t241: F, t125: F, t1353: F, t246: F, t1459: F, t8614: F, t116: F, t8460: F) -> (F, F, F, F, F, F, F, F) {
    let t32285 = t32284 * t1401;
    let t32287 = t8591 * t1405;
    let t32289 = t241 * t1412;
    let t32291 = t246 * t125 * t1353;
    let t32292 = t32289 * t32291;
    let t32293 = t8591 * t32292;
    let t32372 = t1459 * t8614;
    let t32373 = F::new(3.0) * t32372;
    let t32374 = t116 * t8460;
    (t32285, t32287, t32289, t32291, t32292, t32293, t32373, t32374)
}
