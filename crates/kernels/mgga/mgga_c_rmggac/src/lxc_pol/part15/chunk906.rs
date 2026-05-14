//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 906/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk906<F: Float>(t1986: F, t326: F, t559: F, t615: F, t7717: F, t1737: F, t1970: F, t1971: F, t209: F, t476: F, t880: F, t16503: F, t2281: F, t34962: F, t8425: F, t17859: F, t9198: F) -> (F, F, F, F) {
    let t47047 = t1986 * t326 * t559 * t615;
    let t47048 = t7717 * t47047;
    let t47054 = t1970 * t1971 * t880 * t1737 * t476 * t209;
    let t47062 = t16503 * t34962 * t2281 * t8425;
    let t47071 = t17859 * t9198;
    (t47048, t47054, t47062, t47071)
}
