//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2111/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2111(t25878: f64, t98028: f64, t94771: f64, t97814: f64, t1903: f64, t25931: f64, t1882: f64, t2027: f64, t2028: f64, t25889: f64, t25933: f64, t26034: f64, t26084: f64, t27837: f64, t27868: f64, t49376: f64, t543: f64, t545: f64, t5775: f64, t7295: f64, t7296: f64, t7301: f64, t94823: f64, t94880: f64, t94882: f64, t94884: f64, t94887: f64, t94891: f64, t94895: f64, t98290: f64) -> f64 {
    let t98333 = t25878 * t98028;
    let t98338 = t94771 * t97814;
    let t98340 = t25931 * t1903;
    let t98353 = -0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t98290 + 0.8673628188205199462e0_f64 * t27868 * t25931 * t49376 - 0.12851425765524037203e-1_f64 * t94880 - 0.12851425765524037203e-1_f64 * t94882 + 0.2601984143835408805e-1_f64 * t94884 - 0.13170898365871023197e1_f64 * t26084 * t5775 + 0.51405703062096148812e-1_f64 * t94887 - 0.28912093960683998208e-1_f64 * t94891 - 0.34270468708064099208e-1_f64 * t98333 + 0.72280234901709995518e-2_f64 * t94895 + 0.17347256376410398924e1_f64 * t27837 * t25889 - 0.3427046870806409921e-2_f64 * t98338 + 0.52041769129231196772e1_f64 * t94823 * t98340 * t25933 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t26034 * t1903 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t26034 * t1882 * t543;
    t98353
}
