//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 595/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk595<F: Float>(t12161: F, t836: F, t568: F, t739: F, t531: F, t808: F, t314: F, t313: F, t3732: F, t769: F, t1628: F, t3740: F, t1589: F, t3726: F, t10850: F, t10853: F, t10855: F, t10859: F, t2049: F, t2194: F, t2197: F, t317: F, t3733: F, t3736: F, t3741: F, t3746: F, t784: F, t797: F, t813: F, t833: F) -> (F, F, F) {
    let t12162 = t836 * t12161;
    let t12163 = t568 * t12162;
    let t12166 = t739 * t12161;
    let t12167 = t531 * t12166;
    let t12172 = t808 * t12161;
    let t12173 = t568 * t12172;
    let t12176 = t314 * t12161;
    let t12177 = t313 * t12176;
    let t12182 = t769 * t3732;
    let t12185 = t1628 * t3740;
    let t12188 = t1589 * t3726;
    let t12191 = 0.23833659967900284446e0 * t3733 * t784 + 0.23005755572352449806e1 * t2197 * t3746 + 0.23005755572352449806e1 * t833 * t12163 - 0.35750489951850426669e0 * t797 * t12167 - 0.23005755572352449806e1 * t2194 * t3741 - 0.23005755572352449806e1 * t813 * t12173 + 0.35750489951850426669e0 * t12177 * t317 - 0.35750489951850426669e0 * t2049 * t3736 + 0.35750489951850426669e0 * t12182 * t317 - 0.30674340763136599741e1 * t813 * t12185 - 0.23833659967900284446e0 * t797 * t12188 - t10850 + t10853 - t10855 - t10859;
    (t12166, t12176, t12191)
}
