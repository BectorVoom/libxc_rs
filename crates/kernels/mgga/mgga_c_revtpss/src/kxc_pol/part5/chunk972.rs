//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 972/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk972<F: Float>(t1260: F, t3666: F, t12640: F, t225: F, t480: F, t1236: F, t371: F, t676: F, t1235: F, t12627: F, t1226: F, t697: F, t1222: F, t3367: F, t404: F, t1204: F, t3140: F) -> (F, F, F, F, F, F, F, F) {
    let t12956 = t3666 * t1260;
    let t12966 = t12640 * t225;
    let t12967 = t12966 * t480;
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13026 = 1.0 / t404 / t3367;
    let t13032 = t1204 * t3140;
    (t12956, t12966, t12967, t12985, t12987, t13012, t13026, t13032)
}
