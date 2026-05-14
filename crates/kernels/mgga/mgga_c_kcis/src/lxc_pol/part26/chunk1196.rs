//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1196/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1196<F: Float>(t29433: F, t94805: F, t1548: F, t21956: F, t22410: F, t7952: F, t22329: F, t94785: F, t15808: F, t2062: F, t22470: F, t28629: F, t2050: F, t27543: F, t5905: F, t22640: F, t27544: F) -> (F, F, F, F, F, F, F, F) {
    let t102965 = t94805 * t29433;
    let t102967 = t21956 * t1548;
    let t102969 = t7952 * t22410;
    let t102971 = t94785 * t22329;
    let t102973 = t15808 * t2062;
    let t102975 = t28629 * t22470;
    let t102978 = t2050 * t27543 * t5905;
    let t102980 = t27544 * t22640;
    (t102965, t102967, t102969, t102971, t102973, t102975, t102978, t102980)
}
