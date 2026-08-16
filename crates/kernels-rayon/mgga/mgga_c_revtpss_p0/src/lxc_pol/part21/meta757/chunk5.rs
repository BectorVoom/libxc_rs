//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2660/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2660(t1353: f64, t13767: f64, t2661: f64, t48432: f64, t13768: f64, t3889: f64, t3829: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t46949: f64, t47195: f64, t47199: f64, t47216: f64, t47221: f64, t48929: f64, t48937: f64, t48941: f64, t48945: f64, t48947: f64, t5689: f64, t800: f64, t9748: f64) -> f64 {
    let t48951 = t2661 * t13767 * t48432 * t1353;
    let t48955 = t2661 * t13767 * t13768 * t3889;
    let t48965 = -0.38115002106963996168e-4_f64 * t48929 - 0.15246000842785598467e-4_f64 * t46931 + 0.76230004213927992336e-5_f64 * t46934 + 0.76230004213927992336e-5_f64 * t46941 + 0.27107389498472794074e-3_f64 * t46944 - 0.1372140075850703862e-3_f64 * t46947 - 7.0_f64 / 16.0_f64 * t46949 - 0.24009450146119052705e-1_f64 * t48937 + 0.15246000842785598467e-3_f64 * t48941 + 0.76230004213927992338e-4_f64 * t48945 + 0.15117061203111996147e0_f64 * t48947 - 0.85748036236139473944e-3_f64 * t48951 - 0.42874018118069736972e-3_f64 * t48955 - 3.0_f64 / 4.0_f64 * t9748 * t800 * t5689 * t3829 + 0.60023625365297631762e-2_f64 * t47195 - 0.38538502130374707238e-2_f64 * t47199 - 0.8131200449485652516e-3_f64 * t47216 + 0.17149607247227894789e-3_f64 * t47221;
    t48965
}
