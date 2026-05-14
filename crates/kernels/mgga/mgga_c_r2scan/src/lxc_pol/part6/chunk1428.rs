//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1428/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1428<F: Float>(t2049: F, t2768: F, t2061: F, t19013: F, t23750: F, t23751: F, t23752: F, t23753: F, t23759: F, t23761: F, t23763: F, t26922: F, t26924: F, t6044: F, t759: F, t955: F) -> (F, F) {
    let t26926 = t2768 * t2049;
    let t26927 = t2061 * t26926;
    let t26928 = 0.2025780996e0 * t26927;
    let t26929 = t26922 - 0.4051561992e0 * t26924 - t26928 - t23750 - t23751 + t23752 - t19013 - t23753 + t23759 + t23761 - t23763;
    let t26932 = t759 * t955 * t6044;
    (t26929, t26932)
}
