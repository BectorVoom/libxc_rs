//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1036/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1036<F: Float>(t14576: F, t14607: F, t1864: F, t3668: F, t12274: F, t2003: F, t6019: F, t11881: F, t1948: F, t4142: F, t5773: F, t1495: F, t4169: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t15662 = F::cast_from(0.15476481481481481481e-2_f64) * t14576;
    let t15671 = F::cast_from(0.15476481481481481481e-2_f64) * t14607;
    let t15692 = t1864 * t3668;
    let t15800 = t12274 * t2003;
    let t15808 = t6019 * sigma2;
    let t15826 = t11881 * t1948;
    let t15844 = t4142 * t5773;
    let t15865 = t4169 * t1495;
    (t15662, t15671, t15692, t15800, t15808, t15826, t15844, t15865)
}
