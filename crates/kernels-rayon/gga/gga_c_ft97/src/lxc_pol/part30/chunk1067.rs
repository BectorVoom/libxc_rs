//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1067/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1067(t2469: f64, t35678: f64, t33274: f64, t3972: f64, t33489: f64, t3977: f64, t2568: f64, t6187: f64, t6940: f64, t1449: f64, t27889: f64, t1173: f64, t1403: f64, t141552: f64, t193: f64, t27947: f64, t33245: f64, t33248: f64, t33582: f64, t33584: f64, t35263: f64, t5996: f64, t6192: f64, t6745: f64, t6838: f64, t7437: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t151353 = t2469 * t35678;
    let t151355 = t33274 * t3972;
    let t151357 = t3977 * t33489;
    let t151362 = t2568 * t6187 * t6940;
    let t151365 = t2568 * t1449 * t27889;
    let t151380 = t7437 * t27947 / 6.0_f64 - 2.0_f64 * t151353 - 2.0_f64 * t151355 - 2.0_f64 * t151357 + t5996 * t35263 / 3.0_f64 + 8.0_f64 * t151362 + 8.0_f64 * t151365 + t6745 * t33245 - 2.0_f64 / 3.0_f64 * t6745 * t33248 + t1403 * t193 * t33582 * t1173 / 6.0_f64 + t6745 * t33584 / 6.0_f64 + t1403 * t193 * t6838 * t6192 / 3.0_f64 - t141552;
    (t151353, t151355, t151357, t151362, t151365, t151380)
}
