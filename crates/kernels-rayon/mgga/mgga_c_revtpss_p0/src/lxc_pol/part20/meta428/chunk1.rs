//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1608/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608(t12941: f64, t3708: f64, t12269: f64, t12273: f64, t1252: f64, t12781: f64, t12784: f64, t12787: f64, t12789: f64, t3625: f64, t3626: f64, t3714: f64, t44248: f64, t44252: f64, t44260: f64, t44264: f64, t44267: f64, t44270: f64, t44273: f64, t44276: f64, t44278: f64, t5405: f64) -> f64 {
    let t44280 = t3708 * t12941;
    let t44282 = 0.57165357490759649296e-2_f64 * t3625 * t12787 * t12269 * t5405 + 0.28582678745379824648e-2_f64 * t12784 * t12789 - 0.22866142996303859718e-2_f64 * t44248 + 0.3811023832717309953e-3_f64 * t44252 - 0.34299214494455789577e-2_f64 * t12784 * t12781 - 0.34299214494455789577e-2_f64 * t3625 * t3626 * t12273 * t5405 + 0.17149607247227894789e-2_f64 * t44260 * t3714 + 0.2540682555144873302e-3_f64 * t44264 + 0.85748036236139473944e-3_f64 * t44267 * t1252 - 0.57165357490759649296e-3_f64 * t44270 - 0.28582678745379824648e-3_f64 * t44273 + 0.28582678745379824648e-3_f64 * t44276 + 0.17149607247227894789e-2_f64 * t44278 + 0.17149607247227894789e-2_f64 * t44280;
    t44282
}
