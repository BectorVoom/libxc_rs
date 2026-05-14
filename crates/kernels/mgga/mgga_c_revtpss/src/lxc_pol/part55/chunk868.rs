//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 868/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk868<F: Float>(t1310: F, t7983: F, t7315: F, t8108: F, t13648: F, t2107: F, t28683: F, t508: F, t22496: F, t26405: F, t5542: F, t7536: F, t1453: F, t2014: F, t2322: F, t25082: F, t28652: F, t4248: F, t4254: F, t4293: F, t4297: F, t649: F, t651: F, t7359: F, t7378: F, t7969: F, t7984: F, t8065: F, t8075: F) -> (F, F, F, F, F, F, F) {
    let t28704 = t1310 * t7983;
    let t28707 = t8108 * t7315;
    let t28709 = t2107 * t13648;
    let t28711 = t508 * t28683;
    let t28718 = t26405 * t22496;
    let t28727 = t7536 * t5542;
    let t28729 = -t1310 * t7969 + t1453 * t8075 - t2014 * t28707 - t2014 * t28709 - t2014 * t28727 - 2.0 * t2322 * t7984 - 3.0 * t25082 * t28718 - t28652 * t508 - 2.0 * t28704 * t651 - 2.0 * t28711 * t651 - 2.0 * t4248 * t7378 - 2.0 * t4254 * t7984 - 2.0 * t4293 * t7359 - 2.0 * t4297 * t7359 - t649 * t8065;
    (t28704, t28707, t28709, t28711, t28718, t28727, t28729)
}
