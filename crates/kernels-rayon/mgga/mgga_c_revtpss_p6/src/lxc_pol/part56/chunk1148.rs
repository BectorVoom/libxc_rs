//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1148/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1148(t18875: f64, t27799: f64, t126017: f64, t1113: f64, t119706: f64, t119747: f64, t125997: f64, t126006: f64, t126412: f64, t1711: f64, t1940: f64, t2403: f64, t27382: f64, t27773: f64, t27777: f64, t27793: f64, t27802: f64, t27806: f64, t27810: f64, t27817: f64, t31859: f64, t31863: f64, t31876: f64, t33: f64, t33727: f64, t7207: f64, t8490: f64, t8494: f64) -> f64 {
    let t127266 = t27799 * t18875;
    let t127284 = t27799 * t126017;
    let t127287 = 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27810 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27810 + t1940 * t31876 * t27802 - t1940 * t125997 * t7207 / 2.0_f64 - t126006 + t1940 * t31876 * t27806 + t1940 * t31876 * t27817 + t1940 * t31859 * t1711 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t119747 * t27793 + t1940 * t33727 * t1113 / 2.0_f64 - t1940 * t31863 * t27817 / 2.0_f64 - t1940 * t31863 * t27806 / 2.0_f64 + 3.0_f64 * t119706 * t127266 - t1940 * t31863 * t27802 / 2.0_f64 + t1940 * t126412 * t33 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27773 + 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27773 + 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27777 + 2.0_f64 * t27382 * t127284;
    t127287
}
