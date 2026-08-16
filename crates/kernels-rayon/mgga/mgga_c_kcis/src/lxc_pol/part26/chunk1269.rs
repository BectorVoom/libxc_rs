//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1269/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1269(t28884: f64, t28887: f64, t28889: f64, t28891: f64, t27741: f64, t6290: f64, t7671: f64, t20853: f64, t2167: f64, t97601: f64, t26657: f64, t29238: f64, t29249: f64, t29251: f64, t29253: f64, t29256: f64, t29660: f64, t91769: f64, t91772: f64, t91773: f64, t91776: f64, t91777: f64, t91778: f64, t91781: f64, t95271: f64, t97622: f64, t97623: f64, t97624: f64, t97625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99793 = t28884 / 8.0_f64;
    let t99794 = t28887 / 8.0_f64;
    let t99795 = t28889 / 8.0_f64;
    let t99796 = t28891 / 8.0_f64;
    let t99798 = 4.0_f64 * t27741;
    let t99834 = t6290 * t7671;
    let t101750 = t20853 * t2167;
    let t101757 = 2.0_f64 * t97601;
    let t101774 = -t29249 - t91769 + t91772 + t29251 + t29238 + t91773 - t29253 + t97622 + t95271 - t91776 - t97623 + t91777 + t99798 - t91778 + t26657 - t29256 - t29660 - t97624 - t91781 + t97625;
    (t99793, t99794, t99795, t99796, t99834, t101750, t101757, t101774)
}
