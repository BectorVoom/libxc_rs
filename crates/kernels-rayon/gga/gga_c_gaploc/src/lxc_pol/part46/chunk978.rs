//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 978/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk978(t1445: f64, t43213: f64, t833: f64, t1457: f64, t2004: f64, t2087: f64, t2103: f64, t42993: f64, t43307: f64, t43588: f64, t43592: f64, t43597: f64, t43601: f64, t43602: f64, t43603: f64, t43604: f64, t43605: f64, t43606: f64, t43607: f64, t43609: f64, t43611: f64, t43617: f64, t43619: f64, t43620: f64, t43627: f64, t43630: f64, t723: f64) -> f64 {
    let t43636 = 0.11502877786176224903e2_f64 * t833 * t1445 * t43213;
    let t43637 = -0.14300195980740170668e1_f64 * t43588 + t43592 - t43597 + t43601 + t43602 - t43603 - t43604 - t43605 + t43606 - t43607 + 0.38342925953920749676e0_f64 * t43609 + 0.38342925953920749676e0_f64 * t43611 + 0.35750489951850426669e0_f64 * t2004 * t1457 * t42993 + t43617 + t43619 - 0.69017266717057349418e1_f64 * t2087 * t1445 * t43620 * t723 + t43627 + t43630 + 0.71500979903700853338e0_f64 * t2103 * t1457 * t43307 + t43636;
    t43637
}
