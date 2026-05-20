//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 423/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk423<F: Float>(t1248: F, t1287: F, t487: F, t1269: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t460: F, t490: F) -> (F, F, F) {
    let t1288 = t487 * t1248 * t1287;
    let t1291 = t489 * t1269;
    let t1294 = F::cast_from(0.65854491829355115987e0_f64) * t1204 * t490 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1281 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t1288 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1291;
    (t1288, t1291, t1294)
}
