//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1264/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1264(t322: f64, t44684: f64, t44715: f64, t44746: f64, t44778: f64, t44811: f64, t44842: f64, t44873: f64, t12203: f64, t40276: f64, t3250: f64, t3560: f64, t374: f64, t44541: f64, t44544: f64, t44548: f64, t44551: f64, t44554: f64, t44558: f64, t44560: f64, t44562: f64, t44566: f64, t44570: f64, t44574: f64, t44576: f64, t44579: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t44875 = piecewise5(t323, t44684, t331, t44715 + t44746 + t44778 + t44811, t44842 + t44873);
    let t44878 = 5.0_f64 / 8.0_f64 * t40276 * t12203;
    let t44879 = t3250 * t3560 + t374 * t44875 - t44541 - t44544 + t44548 - t44551 + t44554 - t44558 + t44560 + t44562 - t44566 + t44570 + t44574 + t44576 - t44579 - t44878;
    (t44878, t44879)
}
