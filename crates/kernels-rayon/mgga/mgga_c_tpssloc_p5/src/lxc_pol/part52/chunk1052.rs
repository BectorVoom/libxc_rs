//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1052/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1052(t28: f64, t265: f64, t504: f64, t25882: f64, t1409: f64, t1972: f64, t25949: f64, t3966: f64, t52: f64, t607: f64, t6856: f64, t7664: f64, t25890: f64, t113: f64, t2314: f64, t24980: f64, t24983: f64, t24988: f64, t24989: f64, t24993: f64, t24998: f64, t24999: f64, t25005: f64, t25007: f64, t25011: f64, t4073: f64, t4077: f64, t6517: f64, t652: f64, t672: f64, t7472: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t25950 = piecewise3(t505, 0.0_f64, t25882);
    let t25957 = piecewise3(t401, t25949, -t6856 * t1409 / 2.0_f64 - t1972 * t3966 / 2.0_f64 + t25950 * t52 / 2.0_f64 - t7664 * t607 / 2.0_f64);
    let t25958 = t25890 + t25957;
    let t25962 = -t113 * t25958 - 2.0_f64 * t2314 * t7472 - 2.0_f64 * t24980 * t652 - 2.0_f64 * t24983 * t652 - 2.0_f64 * t24999 * t672 - 2.0_f64 * t4073 * t6517 - 2.0_f64 * t4077 * t6517 + t24988 + t24989 + t24993 + t24998 - t25005 - t25007 - t25011;
    (t25958, t25962)
}
