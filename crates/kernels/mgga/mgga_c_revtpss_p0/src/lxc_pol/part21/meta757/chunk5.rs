//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2660/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2660<F: Float>(t1353: F, t13767: F, t2661: F, t48432: F, t13768: F, t3889: F, t3829: F, t46931: F, t46934: F, t46941: F, t46944: F, t46947: F, t46949: F, t47195: F, t47199: F, t47216: F, t47221: F, t48929: F, t48937: F, t48941: F, t48945: F, t48947: F, t5689: F, t800: F, t9748: F) -> F {
    let t48951 = t2661 * t13767 * t48432 * t1353;
    let t48955 = t2661 * t13767 * t13768 * t3889;
    let t48965 = -F::cast_from(0.38115002106963996168e-4_f64) * t48929 - F::cast_from(0.15246000842785598467e-4_f64) * t46931 + F::cast_from(0.76230004213927992336e-5_f64) * t46934 + F::cast_from(0.76230004213927992336e-5_f64) * t46941 + F::cast_from(0.27107389498472794074e-3_f64) * t46944 - F::cast_from(0.1372140075850703862e-3_f64) * t46947 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t46949 - F::cast_from(0.24009450146119052705e-1_f64) * t48937 + F::cast_from(0.15246000842785598467e-3_f64) * t48941 + F::cast_from(0.76230004213927992338e-4_f64) * t48945 + F::cast_from(0.15117061203111996147e0_f64) * t48947 - F::cast_from(0.85748036236139473944e-3_f64) * t48951 - F::cast_from(0.42874018118069736972e-3_f64) * t48955 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9748 * t800 * t5689 * t3829 + F::cast_from(0.60023625365297631762e-2_f64) * t47195 - F::cast_from(0.38538502130374707238e-2_f64) * t47199 - F::cast_from(0.8131200449485652516e-3_f64) * t47216 + F::cast_from(0.17149607247227894789e-3_f64) * t47221;
    t48965
}
