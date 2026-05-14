//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 874/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk874<F: Float>(t2035: F, t32322: F, t6985: F, t7003: F, t7313: F, t8568: F, t32171: F, t508: F, t1310: F, t8454: F, t1932: F, t2007: F, t32161: F, t32162: F, t32301: F, t32303: F, t32305: F, t32307: F, t32309: F, t32312: F, t32316: F, t32320: F, t651: F, t671: F, t6983: F, t7007: F, t7221: F, t8447: F) -> (F,) {
    let t32323 = t32322 * t2035;
    let t32325 = t6985 * t7003;
    let t32329 = t8568 * t7313;
    let t32338 = 2.0 * t32171 * t508;
    let t32340 = 2.0 * t8454 * t1310;
    let t32341 = -t1310 * t8447 - 2.0 * t1932 * t7221 - 2.0 * t2007 * t6983 - t32161 * t508 - 2.0 * t32162 * t671 - 2.0 * t32316 * t651 - 4.0 * t6985 * t7007 - 4.0 * t32301 - 4.0 * t32303 - 4.0 * t32305 - 4.0 * t32307 - 4.0 * t32309 - 4.0 * t32312 - t32320 + 2.0 * t32323 - 4.0 * t32325 + 2.0 * t32329 - t32338 - t32340;
    (t32341,)
}
