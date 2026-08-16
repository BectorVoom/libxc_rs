//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1135/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1135(t28: f64, t1409: f64, t2161: f64, t25949: f64, t27850: f64, t3966: f64, t52: f64, t607: f64, t7402: f64, t8097: f64, t27380: f64, t113: f64, t24988: f64, t24989: f64, t24993: f64, t24998: f64, t25005: f64, t25007: f64, t25011: f64, t25969: f64, t25973: f64, t27290: f64, t27293: f64, t27371: f64, t510: f64, t650: f64, t652: f64, t8103: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t27857 = piecewise3(t401, t25949, -t7402 * t1409 / 2.0_f64 - t2161 * t3966 / 2.0_f64 + t27850 * t52 / 2.0_f64 - t8097 * t607 / 2.0_f64);
    let t27858 = t27380 + t27857;
    let t27860 = -t113 * t27858 - 2.0_f64 * t27290 * t652 - 2.0_f64 * t27293 * t652 - t27371 * t510 - t650 * t8103 + t24988 + t24989 + t24993 + t24998 - t25005 - t25007 - t25011 - t25969 - t25973;
    (t27858, t27860)
}
