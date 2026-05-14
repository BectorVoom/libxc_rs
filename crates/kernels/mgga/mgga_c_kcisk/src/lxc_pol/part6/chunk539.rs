//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 539/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk539<F: Float>(t4041: F, t6020: F, t7914: F, t7917: F, t7920: F, t1235: F, t4054: F, t7931: F, t1242: F, t1248: F, t4065: F, t7736: F, t1249: F, t7740: F, t7744: F, t4049: F, t4061: F, t6066: F, t7932: F) -> (F, F, F, F, F, F, F, F) {
    let t7938 = t4041 + 2.0 / 9.0 * t6020 - 2.0 / 9.0 * t7914 + 2.0 / 3.0 * t7917 - t7920 / 3.0;
    let t7939 = t1235 * t7938;
    let t7945 = t4054 * t7931;
    let t7947 = t1242 * t7938;
    let t7951 = t1248 * t4065 * t7736;
    let t7954 = t1248 * t1249 * t7740;
    let t7957 = t1248 * t1249 * t7744;
    let t7959 = -0.9494625e0 * t7932 + 0.1898925e1 * t7939 + t4049 + 0.19931111111111111111e0 * t6020 - 0.19931111111111111111e0 * t7914 + 0.59793333333333333334e0 * t7917 - 0.29896666666666666667e0 * t7920 + 0.15358125e0 * t7945 + 0.3071625e0 * t7947 + t4061 + 0.21908444444444444444e0 * t6066 - 0.5477111111111111111e-1 * t7951 + 0.32862666666666666666e0 * t7954 - 0.16431333333333333333e0 * t7957;
    (t7938, t7939, t7945, t7947, t7951, t7954, t7957, t7959)
}
