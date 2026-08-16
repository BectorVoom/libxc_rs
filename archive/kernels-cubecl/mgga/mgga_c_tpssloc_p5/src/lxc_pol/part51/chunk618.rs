//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 618/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk618<F: Float>(t1164: F, t4879: F, t1694: F, t3400: F, t1155: F, t3403: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F) -> (F, F, F, F, F) {
    let t4881 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t4879;
    let t4882 = t3400 * t1694;
    let t4883 = t3403 * t1155;
    let t4884 = t4882 * t4883;
    let t4886 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t4884;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    let t4896 = t3431 * t1709;
    (t4881, t4886, t4887, t4889, t4896)
}
