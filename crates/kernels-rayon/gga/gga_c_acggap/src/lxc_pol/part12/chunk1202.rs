//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1202/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1202(t35672: f64, t35678: f64, t35682: f64, t35685: f64, t35702: f64, t31482: f64, t31484: f64, t31487: f64, t31489: f64, t31492: f64, t35668: f64, t35670: f64, t35674: f64, t35676: f64, t35691: f64, t35695: f64, t35698: f64, t35706: f64) -> f64 {
    let t37658 = 0.13719685797782315831e-1_f64 * t35672;
    let t37661 = 0.13719685797782315831e-1_f64 * t35678;
    let t37663 = 0.57165357490759649296e-3_f64 * t35682;
    let t37665 = 11.0_f64 / 24.0_f64 * t35685;
    let t37672 = 0.18868855373762491241e-2_f64 * t35702;
    let t37674 = 0.34299214494455789578e-1_f64 * t35668 + 0.17149607247227894789e-1_f64 * t35670 - t37658 - 0.13719685797782315831e-1_f64 * t35674 - 0.68598428988911579156e-2_f64 * t35676 + t37661 - 0.57165357490759649296e-3_f64 * t31482 - t37663 - 0.51448821741683684367e-2_f64 * t31484 - t37665 + t31487 / 48.0_f64 - 0.916875e-1_f64 * t31489 - 0.183375e0_f64 * t31492 + 0.18868855373762491241e-1_f64 * t35691 - 0.62896184579208304136e-2_f64 * t35695 + 0.94344276868812456204e-2_f64 * t35698 + t37672 + 0.85748036236139473944e-3_f64 * t35706;
    t37674
}
