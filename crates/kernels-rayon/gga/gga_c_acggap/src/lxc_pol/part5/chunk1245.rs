//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1245/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1245(t1861: f64, t3237: f64, t1008: f64, t5529: f64, t1140: f64, t5676: f64, t17167: f64, t176: f64, t20305: f64, t322: f64, t8790: f64, t5534: f64) -> (f64, f64, f64, f64, f64) {
    let t22848 = t3237 * t1861;
    let t22850 = t1008 * t5529;
    let t22865 = t1140 * t5676;
    let t22880 = t17167 * t176 * t8790 * t20305 * t322;
    let t22882 = t1008 * t5534;
    (t22848, t22850, t22865, t22880, t22882)
}
