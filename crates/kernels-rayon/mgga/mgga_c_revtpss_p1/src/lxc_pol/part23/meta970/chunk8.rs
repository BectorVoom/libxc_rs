//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3278/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3278(t22813: f64, t547: f64, t807: f64, t9941: f64, t1413: f64, t22809: f64, t13767: f64, t1868: f64, t2661: f64, t74012: f64, t13789: f64, t13790: f64, t21990: f64, t22079: f64, t47262: f64, t49008: f64, t49012: f64, t49030: f64, t49057: f64, t5671: f64, t5673: f64, t73847: f64, t74579: f64, t74583: f64, t74585: f64, t74589: f64, t74598: f64, t74602: f64, t74606: f64, t85625: f64) -> f64 {
    let t86165 = t807 * t547 * t9941 * t22813;
    let t86169 = t807 * t547 * t1413 * t22809;
    let t86183 = t2661 * t13767 * t74012 * t1868;
    let t86198 = 0.85748036236139473942e-3_f64 * t86165 + 0.28582678745379824648e-4_f64 * t86169 + 0.12862205435420921092e-2_f64 * t5671 * t5673 * t73847 * t13790 + 0.12862205435420921092e-2_f64 * t5671 * t5673 * t22079 * t21990 - 0.5421477899694558815e-4_f64 * t49008 - 0.30492001685571196934e-4_f64 * t49012 - 0.42874018118069736973e-3_f64 * t86183 + 455.0_f64 / 216.0_f64 * t49030 - 0.10289764348336736873e-1_f64 * t5671 * t13789 * t13790 * t85625 + 0.32528867398167352889e-3_f64 * t47262 + 0.17149607247227894789e-3_f64 * t74579 + 0.12862205435420921092e-3_f64 * t74583 - 0.17006693853500995666e-1_f64 * t74585 + 0.42874018118069736972e-3_f64 * t74589 - 0.17149607247227894789e-3_f64 * t74598 - 0.17149607247227894789e-3_f64 * t74602 - 0.85748036236139473944e-4_f64 * t74606 + t49057;
    t86198
}
