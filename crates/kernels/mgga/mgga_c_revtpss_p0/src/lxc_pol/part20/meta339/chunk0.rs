//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1264/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1264<F: Float>(t15687: F, t3088: F, t3317: F, t12131: F, t3095: F, t1087: F, t11773: F, t372: F, t4801: F, t1062: F, t11940: F, t11788: F) -> (F, F, F, F, F, F, F) {
    let t15688 = t3088 * t15687;
    let t15689 = t3317 * t15688;
    let t15692 = t12131 * t3095;
    let t15700 = t1087 * t11773;
    let t15701 = t372 * t4801;
    let t15716 = t11940 * t1062;
    let t15725 = t11788 * t1062;
    (t15688, t15689, t15692, t15700, t15701, t15716, t15725)
}
