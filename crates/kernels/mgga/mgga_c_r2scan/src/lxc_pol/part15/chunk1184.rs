//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1184/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1184<F: Float>(t40217: F, t10899: F, t11770: F, t2201: F, t2834: F, t3316: F, t10820: F, t26088: F, t20407: F, t2161: F, t2841: F, t625: F) -> (F, F, F, F, F) {
    let t40218 = F::cast_from(0.10975748638225852664e-1_f64) * t40217;
    let t40220 = t2201 * t10899 * t11770;
    let t40222 = t2834 * t3316;
    let t40223 = F::cast_from(0.23115257973478049502e0_f64) * t40222;
    let t40224 = t26088 * t10820;
    let t40228 = t2161 * t20407 * t2841 * t625;
    (t40218, t40220, t40223, t40224, t40228)
}
