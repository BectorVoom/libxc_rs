//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2032/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2032(t102854: f64, t103586: f64, t14365: f64, t14468: f64, t18875: f64, t1940: f64, t2071: f64, t2403: f64, t2408: f64, t2430: f64, t26585: f64, t26590: f64, t27375: f64, t27384: f64, t28456: f64, t28460: f64, t4537: f64, t51780: f64, t61102: f64, t61203: f64, t63164: f64, t7432: f64, t775: f64, t8020: f64, t8031: f64, t890: f64, t95976: f64, t98651: f64, t98779: f64) -> f64 {
    let t103706 = -2.0_f64 * t102854 * t1940 * t890 + 2.0_f64 * t103586 * t1940 * t2408 - 6.0_f64 * t14365 * t2403 * t28460 + 3.0_f64 * t14468 * t2071 * t2403 - 6.0_f64 * t18875 * t2403 * t26585 - 2.0_f64 * t1940 * t26585 * t4537 + 4.0_f64 * t1940 * t26590 * t63164 + 2.0_f64 * t1940 * t26590 * t98779 + 4.0_f64 * t1940 * t27384 * t95976 + 3.0_f64 * t2403 * t2430 * t8020 - 6.0_f64 * t2403 * t26585 * t27375 + 6.0_f64 * t2403 * t28456 * t775 - 6.0_f64 * t2403 * t61102 * t7432 - 3.0_f64 * t2403 * t61203 * t7432 - 3.0_f64 * t2403 * t7432 * t98651 + 6.0_f64 * t51780 * t8031;
    t103706
}
