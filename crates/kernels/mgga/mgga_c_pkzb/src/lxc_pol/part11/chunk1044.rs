//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1044/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1044<F: Float>(t2036: F, t26659: F, t2156: F, t9716: F, t3698: F, t6065: F, t2242: F, t9837: F, t3932: F, t6362: F, t10282: F, t914: F, t10334: F, t6455: F, t10066: F, t3206: F, t6475: F) -> (F, F, F, F, F, F, F, F) {
    let t26695 = t2036 * t26659;
    let t26780 = t9716 * t2156;
    let t26809 = t3698 * t6065;
    let t26880 = t9837 * t2242;
    let t26901 = t3932 * t6362;
    let t26927 = t914 * t10282;
    let t26948 = t6455 * t10334;
    let t26970 = t3206 * t6475 * t10066;
    (t26695, t26780, t26809, t26880, t26901, t26927, t26948, t26970)
}
