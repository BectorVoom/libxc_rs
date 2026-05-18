//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1009/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1009<F: Float>(t1652: F, t1970: F, t1971: F, t209: F, t476: F, t515: F, t7244: F, t8432: F, t1475: F, t839: F, t880: F, t236: F, t794: F, t9188: F) -> (F, F, F, F) {
    let t42099 = t1970 * t1971 * t515 * t1652 * t476 * t209;
    let t42101 = t7244 * t8432;
    let t42109 = t1970 * t1971 * t880 * t1475 * t839;
    let t42114 = t1970 * t9188 * t236 * t1475 * t794;
    (t42099, t42101, t42109, t42114)
}
