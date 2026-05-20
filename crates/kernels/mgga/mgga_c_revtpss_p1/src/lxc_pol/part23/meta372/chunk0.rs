//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1701/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1701<F: Float>(t1040: F, t15816: F, t1647: F, t3140: F, t3149: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F) -> (F, F, F, F, F, F) {
    let t15817 = t15816 * t1040;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15827 = t247 * t11921 * t4757;
    let t15829 = F::cast_from(0.57165357490759649296e-3_f64) * t4837 * t15827;
    let t15830 = t1659 * t3105;
    (t15817, t15822, t15823, t15827, t15829, t15830)
}
