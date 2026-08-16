//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1077/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1077(t12920: f64, t5268: f64, t1042: f64, t11231: f64, t1261: f64, t12847: f64, t12853: f64, t12855: f64, t12858: f64, t12862: f64, t12866: f64, t12868: f64, t12872: f64, t12876: f64, t12882: f64, t12887: f64, t12890: f64, t12893: f64, t12895: f64, t12900: f64, t12902: f64, t12905: f64, t12907: f64, t12910: f64, t12912: f64, t12918: f64, t3711: f64, t3718: f64, t484: f64, t5331: f64, t5340: f64) -> f64 {
    let t12921 = t5268 * t12920;
    let t12922 = t1042 * t12921;
    let t12925 = t5268 * t11231;
    let t12926 = t1042 * t12925;
    let t12929 = 0.42874018118069736972e-3_f64 * t5331 * t12847 + t12853 - 0.12862205435420921092e-2_f64 * t12855 * t12858 - 0.64311027177104605458e-3_f64 * t3718 * t12862 + 0.85748036236139473944e-3_f64 * t12866 * t12868 + 0.12862205435420921092e-2_f64 * t5340 * t12872 - 0.64311027177104605458e-3_f64 * t5331 * t12876 + 0.95275595817932748825e-4_f64 * t12882 + 0.47637797908966374413e-3_f64 * t12887 + 0.21437009059034868486e-3_f64 * t12890 * t484 - 0.14291339372689912324e-3_f64 * t12893 + 0.42874018118069736972e-3_f64 * t12895 + t12900 + 0.85748036236139473944e-3_f64 * t12902 - 0.14291339372689912324e-3_f64 * t12905 + 0.85748036236139473944e-3_f64 * t12907 + 0.12862205435420921092e-2_f64 * t12910 * t12912 - 0.85748036236139473944e-3_f64 * t12918 + 0.85748036236139473944e-3_f64 * t3711 * t12922 - 0.85748036236139473944e-3_f64 * t1261 * t12926;
    t12929
}
