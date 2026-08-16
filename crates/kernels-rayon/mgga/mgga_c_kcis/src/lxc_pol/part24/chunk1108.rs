//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1108/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1108(t1646: f64, t1856: f64, t28110: f64, t5310: f64, t6842: f64, t7773: f64, t5329: f64) -> (f64, f64, f64, f64) {
    let t29121 = t1646 * t1856;
    let t29122 = t28110 * t29121;
    let t29123 = t5310 * t29122;
    let t29126 = t7773 * t6842;
    let t29127 = t5329 * t29126;
    (t29122, t29123, t29126, t29127)
}
