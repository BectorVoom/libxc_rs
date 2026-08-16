//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2549/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549<F: Float>(t11821: F, t140: F, t42793: F, t4892: F, t4895: F, t4899: F, t4901: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F) -> (F, F, F, F, F) {
    let t53972 = t140 * t11821;
    let t54036 = t4892 * t42793 * t4895;
    let t54037 = F::cast_from(0.28582678745379824648e-3_f64) * t54036;
    let t54078 = t4899 * t42793 * t4901;
    let t54079 = F::cast_from(0.14291339372689912324e-3_f64) * t54078;
    let t54118 = t1011 * t2438 * t1655;
    let t54122 = t1011 * t697 * t1014 * t4579;
    (t53972, t54037, t54079, t54118, t54122)
}
