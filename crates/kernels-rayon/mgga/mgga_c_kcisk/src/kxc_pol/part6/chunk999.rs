//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 999/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk999(t30402: f64, t30474: f64, t67: f64, t2152: f64, t8010: f64, t13776: f64, t7831: f64) -> (f64, f64, f64, f64) {
    let t30476 = t67 * (t30402 + t30474);
    let t30489 = t8010 * t2152;
    let t30490 = t13776 * t30489;
    let t30494 = t2152 * t7831;
    (t30476, t30489, t30490, t30494)
}
