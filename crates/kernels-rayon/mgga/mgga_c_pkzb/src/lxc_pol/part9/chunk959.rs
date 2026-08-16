//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 959/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk959(t261: f64, t7510: f64, t7335: f64, t5522: f64, t5525: f64, t5812: f64, t7352: f64, t7357: f64, t1918: f64, t1957: f64, t1972: f64, t248: f64, t2829: f64, t704: f64, t714: f64, t723: f64, t7446: f64, t7447: f64, t7475: f64, t7478: f64, t7485: f64, t7486: f64, t7491: f64, t7493: f64, t7494: f64, t7504: f64) -> (f64, f64, f64) {
    let t7511 = t7510 * t261;
    let t7516 = 0.34246666666666666666e-1_f64 * t7335;
    let t7518 = -t5812 + 0.45662222222222222222e-1_f64 * t5522 - 0.17123333333333333333e-1_f64 * t5525 + 0.22831111111111111111e-1_f64 * t7357 - t7516 + 0.5137e-1_f64 * t7352;
    let t7521 = -t7446 + 2.0_f64 * t7447 * t704 + 0.5848223622634646207e0_f64 * t714 * t7475 + 0.11696447245269292414e1_f64 * t7478 * t723 + 0.5848223622634646207e0_f64 * t2829 * t1972 + t7485 - 2.0_f64 * t7486 * t1918 - t7491 - t7493 - 0.11696447245269292414e1_f64 * t7494 * t1957 + t7504 - 0.19751673498613801407e-1_f64 * t7511 - 0.310907e-1_f64 * t7518 * t248;
    (t7511, t7518, t7521)
}
