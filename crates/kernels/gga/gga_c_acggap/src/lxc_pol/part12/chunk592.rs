//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 592/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk592<F: Float>(t174: F, t4875: F, t386: F, t428: F, t3228: F, t532: F, t1008: F, t1569: F, t3266: F, t422: F, t530: F, t3670: F, t542: F, t540: F, t537: F, t335: F, t367: F, t418: F, t4769: F, t4773: F, t4777: F, t4781: F, t4785: F, t4787: F, t4791: F, t4840: F, t4843: F, t4846: F, t4849: F, t4853: F) -> (F, F, F, F, F) {
    let t4876 = t174 * t4875;
    let t4878 = t386 * t428 * t4876;
    let t4881 = t3228 * t532;
    let t4884 = 0.17149607247227894789e-2 * t1008 * t1569;
    let t4886 = t422 * t3266 * t530;
    let t4889 = t3670 * t532;
    let t4891 = t3670 * t542;
    let t4894 = t386 * t3266 * t540;
    let t4897 = t3670 * t537;
    let t4899 = -t335 * t4769 / 48.0 - t367 * t4773 / 48.0 - t335 * t4777 / 24.0 - t335 * t4781 / 48.0 + t4785 - t335 * t4787 / 48.0 - t367 * t4791 / 48.0 - t367 * t4840 / 96.0 + 0.80031500487063509014e-2 * t4843 + t4846 + 0.42874018118069736972e-3 * t418 * t4849 + 0.85748036236139473944e-3 * t418 * t4853 + 0.42874018118069736972e-3 * t418 * t4878 - 0.85748036236139473944e-3 * t4881 - t4884 - 0.17149607247227894789e-2 * t418 * t4886 - 0.22675591804667994221e-1 * t4889 - 0.11337795902333997111e-1 * t4891 - 0.85748036236139473944e-3 * t418 * t4894 + 0.11337795902333997111e-1 * t4897;
    (t4876, t4878, t4886, t4894, t4899)
}
