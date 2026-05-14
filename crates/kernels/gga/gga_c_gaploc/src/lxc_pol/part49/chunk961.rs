//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 961/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk961<F: Float>(t47311: F, t568: F, t808: F, t813: F, t13883: F, t1589: F, t797: F, t13880: F, t784: F, t13884: F, t2049: F, t739: F, t531: F, t43890: F, t43891: F, t43895: F, t43901: F, t47405: F, t47406: F, t47408: F) -> (F, F) {
    let t47412 = 0.23005755572352449806e1 * t813 * t568 * t808 * t47311;
    let t47415 = 0.23833659967900284446e0 * t797 * t1589 * t13883;
    let t47417 = 0.23833659967900284446e0 * t13880 * t784;
    let t47419 = 0.35750489951850426669e0 * t2049 * t13884;
    let t47420 = t739 * t47311;
    let t47423 = 0.35750489951850426669e0 * t797 * t531 * t47420;
    let t47425 = t43890 - t43891 + t47405 + t47406 - t47408 - t47412 - t47415 + t47417 - t47419 - t47423 - t43895 + 0.23833659967900284446e0 * t43901;
    (t47420, t47425)
}
