//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1473/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1473<F: Float>(t2105: F, t7758: F, t2029: F, t7945: F, t2022: F, t7961: F, t116021: F, t116026: F, t116032: F, t116038: F, t116044: F, t1396: F, t1404: F, t1852: F, t1858: F, t2023: F, t27286: F, t31782: F, t31820: F, t33628: F, t33662: F, t5364: F, t7003: F, t7240: F, t7759: F, t8660: F) -> F {
    let t122860 = t7758 * t2105;
    let t122862 = t7945 * t2029;
    let t122864 = t2022 * t7961;
    let t122870 = t1396 * t33662 + t1404 * t33628 + t1852 * t31820 + t1858 * t31782 + t2023 * t27286 + t5364 * t8660 + t7003 * t7961 + t7240 * t7759 + t116021 + t116026 + t116032 + t116038 + t116044 + t122860 + t122862 + t122864;
    t122870
}
