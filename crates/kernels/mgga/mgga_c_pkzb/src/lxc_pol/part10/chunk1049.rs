//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1049/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1049<F: Float>(t3418: F, t5257: F, t1721: F, t3441: F, t600: F, t179: F, t3410: F, t5391: F, t8914: F, t164: F, t568: F, t2600: F, t2639: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8946 = t5257 * t3418;
    let t8948 = t3441 * t1721;
    let t8949 = t8948 * t600;
    let t8950 = t179 * t8949;
    let t8953 = t3410 * t5391;
    let t8954 = t8953 * t600;
    let t8955 = t179 * t8954;
    let t8958 = t8914 * t600;
    let t8959 = t179 * t8958;
    let t8962 = t3410 * t164;
    let t8964 = t179 * t8962 * t568;
    let t8967 = t2600 * t2639;
    let t8968 = t179 * t8967;
    let t8971 = t3410 * t600;
    let t8972 = t8971 * t164;
    let t8973 = t179 * t8972;
    (t8946, t8948, t8949, t8950, t8953, t8954, t8955, t8958, t8959, t8962, t8964, t8967, t8968, t8971, t8972, t8973)
}
