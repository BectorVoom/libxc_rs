//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 979/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk979(t10332: f64, t10378: f64, t10413: f64, t10460: f64, t10504: f64, t10560: f64, t10607: f64, t10621: f64, t502: f64, t3513: f64, t617: f64, t1022: f64, t935: f64) -> (f64, f64, f64, f64) {
    let t10624 = t10332 + t10378 + t10413 + t10460 + t10504 + t10560 + t10607 + t10621;
    let t10625 = t502 * t10624;
    let t10626 = t617 * t3513;
    let t10627 = t1022 * t935;
    (t10624, t10625, t10626, t10627)
}
