//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1228/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1228(t107923: f64, t1113: f64, t127227: f64, t127284: f64, t127596: f64, t1940: f64, t2403: f64, t25759: f64, t26425: f64, t26585: f64, t27773: f64, t27777: f64, t27810: f64, t28472: f64, t32505: f64, t33888: f64, t34080: f64, t34090: f64, t34145: f64, t7200: f64, t7432: f64, t8657: f64, t94245: f64, t95511: f64) -> f64 {
    let t128183 = 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27773 + 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27777 - 3.0_f64 / 2.0_f64 * t26425 * t25759 * t127596 + t28472 * t127284 - t1940 * t26585 * t33888 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27810 + 3.0_f64 / 2.0_f64 * t2403 * t34080 * t7200 - 3.0_f64 / 2.0_f64 * t26425 * t94245 * t34090 - 3.0_f64 / 2.0_f64 * t95511 * t34145 + t28472 * t107923 * t32505 + t1940 * t34080 * t1113 / 2.0_f64 - t1940 * t7432 * t127227 / 2.0_f64;
    t128183
}
