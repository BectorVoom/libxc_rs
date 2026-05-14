//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 876/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk876<F: Float>(t3806: F, t6230: F, t6233: F, t889: F, t898: F, t3147: F, t3162: F, t2295: F, t3819: F, t891: F, t3840: F, t2317: F, t3161: F, t237: F, t9974: F, t10029: F, t9844: F, t9846: F, t9849: F, t9852: F, t9855: F, t9858: F, t9862: F, t9866: F, t9870: F, t9978: F, t9980: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10168 = t6230 * t3806;
    let t10169 = t6233 * t889;
    let t10170 = t10168 * t10169;
    let t10172 = 0.10254018858216406658e4 * t898 * t10170;
    let t10174 = 0.34631718211362927517e2 * t3147 * t3162;
    let t10175 = t2295 * t3819;
    let t10176 = t10175 * t891;
    let t10178 = 0.11696447245269292414e1 * t898 * t10176;
    let t10179 = t3840 * t891;
    let t10181 = 0.35089341735807877242e1 * t898 * t10179;
    let t10182 = t2317 * t3819;
    let t10183 = t10182 * t3161;
    let t10185 = 0.17315859105681463759e2 * t898 * t10183;
    let t10187 = 0.19751673498613801407e-1 * t237 * t9974;
    let t10188 = -t9844 + t9846 + t9849 - t9852 - t9855 - t9858 + t9862 + t9866 + t9870 - t10172 - t10174 + t10178 - t10181 - t10185 + t10187 + t9978 + t9980 - t10029;
    (t10168, t10169, t10170, t10172, t10174, t10176, t10178, t10179, t10181, t10182, t10183, t10185, t10187, t10188)
}
