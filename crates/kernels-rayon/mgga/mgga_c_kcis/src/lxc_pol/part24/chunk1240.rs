//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1240/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1240(t1020: f64, t18530: f64, t7718: f64, t1856: f64, t26996: f64, t5329: f64, t5336: f64, t1267: f64, t30066: f64, t6774: f64, t26975: f64, t5341: f64) -> (f64, f64, f64, f64) {
    let t100229 = t1020 * t7718 * t18530;
    let t100235 = t5329 * t26996 * t5336 * t1856;
    let t100244 = t5329 * t30066 * t6774 * t1267;
    let t100257 = t5329 * t26975 * t1856 * t5341;
    (t100229, t100235, t100244, t100257)
}
