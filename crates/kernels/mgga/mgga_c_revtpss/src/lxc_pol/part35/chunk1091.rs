//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1091/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1091<F: Float>(t114986: F, t115406: F, t115962: F, t116006: F, t111320: F, t114905: F, t117: F, t1518: F, t1916: F, t1918: F, t2113: F, t2115: F, t22633: F, t25055: F, t25063: F, t25066: F, t25069: F, t28986: F, t30637: F, t30651: F, t30654: F, t30657: F, t30660: F, t34359: F, t572: F, t573: F, t5883: F, t5920: F, t6941: F, t6945: F, t6948: F, t7553: F, t7983: F, t8118: F, t8124: F, t8127: F) -> (F, F) {
    let t116008 = t114986 + t115406 + t115962 + t116006;
    let t116023 = 3.0 * t572 * t117 * t114905 + 18.0 * t8118 * t6945 + 18.0 * t2113 * t25066 + 9.0 * t1916 * t30660 + 9.0 * t8118 * t6948 + 6.0 * t572 * t7553 * t22633 + 6.0 * t2113 * t25063 + 18.0 * t572 * t111320 * t1518 + 18.0 * t572 * t28986 * t5920 + 9.0 * t30637 * t1918 + 18.0 * t1916 * t30651 + 36.0 * t1916 * t30654 + 18.0 * t1916 * t30657 + 18.0 * t6941 * t8124 + param_d * t116008 * t573 + 18.0 * t572 * t34359 * t5920 + 18.0 * t572 * t5883 * t7983 + 9.0 * t6941 * t8127 + 3.0 * t2113 * t25069 + 3.0 * t25055 * t2115;
    (t116008, t116023)
}
