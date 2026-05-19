//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 935/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk935<F: Float>(t10182: F, t3161: F, t898: F, t237: F, t9974: F, t10029: F, t10172: F, t10174: F, t10178: F, t10181: F, t9844: F, t9846: F, t9849: F, t9852: F, t9855: F, t9858: F, t9862: F, t9866: F, t9870: F, t9978: F, t9980: F) -> (F, F, F, F) {
    let t10183 = t10182 * t3161;
    let t10185 = F::cast_from(0.17315859105681463759e2_f64) * t898 * t10183;
    let t10187 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t9974;
    let t10188 = -t9844 + t9846 + t9849 - t9852 - t9855 - t9858 + t9862 + t9866 + t9870 - t10172 - t10174 + t10178 - t10181 - t10185 + t10187 + t9978 + t9980 - t10029;
    (t10183, t10185, t10187, t10188)
}
