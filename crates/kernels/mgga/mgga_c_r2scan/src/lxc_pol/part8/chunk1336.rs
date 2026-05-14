//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1336/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1336<F: Float>(t32314: F, t6363: F, t2115: F, t2155: F, t10308: F, t255: F, t537: F, t571: F, t10121: F, t1600: F, t10101: F, t19797: F, t19801: F, t24055: F, t24068: F, t24076: F, t24097: F, t27998: F, t28002: F, t28007: F, t576: F, t6465: F, t8240: F, t9112: F) -> (F, F) {
    let t32696 = t32314 * t6363;
    let t32697 = t2115 * t32696;
    let t32698 = t2155 * t32697;
    let t32705 = t571 * t537 * t10308 * t255;
    let t32708 = t1600 * t10121;
    let t32716 = -0.26832961483302653302e-2 * t24055 - 0.52690178912667028302e0 * t24068 - 0.29272321618148349057e-1 * t32698 - t24076 + 0.1047928639570397803e0 * t27998 + 0.1047928639570397803e0 * t28002 + 0.82318114786693894983e-2 * t28007 - 0.43341108700271342816e-1 * t32705 * t576 + 0.64025200389650807209e-1 * t32708 + 0.26004665220162805689e0 * t6465 * t10101 + 0.39006997830244208535e0 * t8240 * t9112 - 0.73613752582167450608e0 * t19797 - 0.22084125774650235182e1 * t19801 - t24097;
    (t32697, t32716)
}
