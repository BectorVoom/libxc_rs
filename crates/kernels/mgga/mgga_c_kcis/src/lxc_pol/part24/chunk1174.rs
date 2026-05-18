//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1174/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1174<F: Float>(t11000: F, t1268: F, t1241: F, t209: F, t7787: F, t1094: F, t283: F, t1130: F, t46978: F, t7788: F, t7795: F, t92748: F) -> (F, F, F, F, F, F, F) {
    let t92787 = t11000 * t1268;
    let t92794 = t1241 * t209;
    let t92795 = t7787 * t92794;
    let t92807 = t1094 * t283;
    let t92808 = t92807 * t1130;
    let t92896 = t7788 * t46978 * t7795;
    let t92898 = t7788 * t92748;
    (t92787, t92794, t92795, t92807, t92808, t92896, t92898)
}
