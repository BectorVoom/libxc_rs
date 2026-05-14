//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1057/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1057<F: Float>(t10856: F, t8071: F, t37769: F, t7620: F, t10899: F, t11770: F, t2201: F, t2834: F, t3316: F, t10820: F, t26088: F, t20407: F, t2161: F, t2841: F, t625: F, t40197: F, t40201: F, t40204: F, t40207: F, t40210: F, t40213: F) -> (F,) {
    let t40215 = t10856 * t8071;
    let t40216 = 0.97574405393827830186e-2 * t40215;
    let t40217 = t37769 * t7620;
    let t40218 = 0.10975748638225852664e-1 * t40217;
    let t40220 = t2201 * t10899 * t11770;
    let t40222 = t2834 * t3316;
    let t40223 = 0.23115257973478049502e0 * t40222;
    let t40224 = t26088 * t10820;
    let t40228 = t2161 * t20407 * t2841 * t625;
    let t40230 = -0.32927245914677557994e0 * t40197 - 0.95219938395347901943e-2 * t40201 + 0.2600466522016280569e0 * t40204 - 0.2600466522016280569e0 * t40207 + 0.10975748638225852664e0 * t40210 - 0.10401866088065122276e1 * t40213 - t40216 - t40218 + 0.22511059664845582436e0 * t40220 - t40223 - 0.43663693315433241792e-2 * t40224 + 0.16262400898971305031e-3 * t40228;
    (t40230,)
}
