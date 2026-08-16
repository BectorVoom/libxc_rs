//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 631/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk631(t3670: f64, t542: f64, t3266: f64, t386: f64, t540: f64, t537: f64, t335: f64, t367: f64, t418: f64, t4769: f64, t4773: f64, t4777: f64, t4781: f64, t4785: f64, t4787: f64, t4791: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4853: f64, t4878: f64, t4881: f64, t4884: f64, t4886: f64, t4889: f64) -> (f64, f64) {
    let t4891 = t3670 * t542;
    let t4894 = t386 * t3266 * t540;
    let t4897 = t3670 * t537;
    let t4899 = -t335 * t4769 / 48.0_f64 - t367 * t4773 / 48.0_f64 - t335 * t4777 / 24.0_f64 - t335 * t4781 / 48.0_f64 + t4785 - t335 * t4787 / 48.0_f64 - t367 * t4791 / 48.0_f64 - t367 * t4840 / 96.0_f64 + 0.80031500487063509014e-2_f64 * t4843 + t4846 + 0.42874018118069736972e-3_f64 * t418 * t4849 + 0.85748036236139473944e-3_f64 * t418 * t4853 + 0.42874018118069736972e-3_f64 * t418 * t4878 - 0.85748036236139473944e-3_f64 * t4881 - t4884 - 0.17149607247227894789e-2_f64 * t418 * t4886 - 0.22675591804667994221e-1_f64 * t4889 - 0.11337795902333997111e-1_f64 * t4891 - 0.85748036236139473944e-3_f64 * t418 * t4894 + 0.11337795902333997111e-1_f64 * t4897;
    (t4894, t4899)
}
