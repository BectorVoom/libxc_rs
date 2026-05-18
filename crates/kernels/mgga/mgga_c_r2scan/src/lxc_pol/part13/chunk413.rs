//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 413/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk413<F: Float>(t166: F, t1678: F, t159: F, t15: F, t26: F, t20: F, t21: F, t263: F, t695: F) -> (F, F, F, F, F, F, F, F) {
    let t1679 = t166 * t1678;
    let t1680 = t159 * t1679;
    let t1683 = F::new(1.0) / t15 / t26 / F::new(4.0);
    let t1684 = t1683 * t20;
    let t1685 = t21 * t263;
    let t1686 = t1684 * t1685;
    let t1688 = F::new(0.42340699333333333333e-3) * t1680 * t1686;
    let t1691 = t695 * t695;
    (t1679, t1680, t1683, t1684, t1685, t1686, t1688, t1691)
}
