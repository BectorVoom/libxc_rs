//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 841/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk841<F: Float>(t13141: F, t2464: F, t2684: F, t13097: F, t13098: F, t13101: F, t13102: F, t13105: F, t13106: F, t13109: F, t13110: F, t1589: F, t1628: F, t2049: F, t2194: F, t2197: F, t313: F, t314: F, t317: F, t43081: F, t43082: F, t43527: F, t43529: F, t43567: F, t43569: F, t43571: F, t43575: F, t43579: F, t531: F, t568: F, t769: F, t784: F, t797: F, t808: F, t813: F, t833: F, t836: F) -> (F,) {
    let t43581 = t2684 * t2464 * t13141;
    let t43582 = 0.63904876589867916128e-1 * t43581;
    let t43583 = t43527 - 0.29792074959875355558e-1 * t43529 - 0.35750489951850426669e0 * t2049 * t13102 - 0.35750489951850426669e0 * t797 * t531 * t43082 + 0.23005755572352449806e1 * t833 * t568 * t836 * t43081 + 0.23005755572352449806e1 * t2197 * t13110 + 0.23833659967900284446e0 * t13098 * t784 + 0.35750489951850426669e0 * t313 * t314 * t43081 * t317 - 0.23005755572352449806e1 * t2194 * t13106 - 0.23005755572352449806e1 * t813 * t568 * t808 * t43081 - 0.30674340763136599741e1 * t813 * t1628 * t13105 - 0.23833659967900284446e0 * t797 * t1589 * t13101 + 0.30674340763136599741e1 * t833 * t1628 * t13109 + 0.35750489951850426669e0 * t769 * t13097 * t317 + t43567 + t43569 + t43571 - t43575 + t43579 - t43582;
    (t43583,)
}
