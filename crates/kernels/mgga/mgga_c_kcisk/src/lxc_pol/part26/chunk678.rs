//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 678/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk678<F: Float>(t1254: F, t7959: F, t4083: F, t7927: F, t4087: F, t6020: F, t7914: F, t7917: F, t7920: F, t2141: F, t1275: F, t4100: F, t4108: F, t4115: F, t6066: F, t7932: F, t7939: F, t7945: F, t7947: F, t7951: F, t7954: F, t7957: F) -> (F, F, F, F, F, F) {
    let t7960 = t7959 * t1254;
    let t7963 = t7927 * t4083;
    let t7970 = t4087 + 0.61805555555555555556e-2 * t6020 - 0.61805555555555555555e-2 * t7914 + 0.18541666666666666667e-1 * t7917 - 0.92708333333333333333e-2 * t7920;
    let t7976 = t2141 * t2141;
    let t7978 = t4100 * t7976 * t1275;
    let t7993 = -0.1294625e1 * t7932 + 0.258925e1 * t7939 + t4108 + 0.20128333333333333334e0 * t6020 - 0.20128333333333333333e0 * t7914 + 0.60385e0 * t7917 - 0.301925e0 * t7920 + 0.82524375e-1 * t7945 + 0.16504875e0 * t7947 + t4115 + 0.22076e0 * t6066 - 0.5519e-1 * t7951 + 0.33114e0 * t7954 - 0.16557e0 * t7957;
    (t7960, t7963, t7970, t7976, t7978, t7993)
}
