//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1136/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1136(t19655: f64, t19682: f64, t98: f64, t124: f64, t1545: f64, t2605: f64, t1548: f64, t16476: f64, t16193: f64, t16230: f64, t16273: f64, t16275: f64, t16280: f64, t16283: f64, t16287: f64, t16290: f64, t16481: f64, t16486: f64, t16489: f64, t192: f64, t19621: f64, t19624: f64, t19626: f64, t19628: f64, t2718: f64, t568: f64, t6853: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19684 = (t19655 + t19682) * t98;
    let t19686 = 0.19751673498613801407e-1_f64 * t19684 * t124;
    let t19687 = t1545 * t2605;
    let t19688 = 36.0_f64 * t19687;
    let t19690 = 96.0_f64 * t1548 * t2605;
    let t19691 = 0.10526802520742363173e2_f64 * t16476;
    let t19692 = 18.0_f64 * t192 * t2718 * t568 * t6853 - t16193 - t16230 - t16273 + t16275 - t16280 + t16283 + t16287 - t16290 + t16481 - t16486 - t16489 - t19621 + t19624 + t19626 + t19628 + t19686 + t19688 - t19690 + t19691;
    (t19684, t19686, t19688, t19690, t19691, t19692)
}
