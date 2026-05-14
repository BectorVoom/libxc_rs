//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1072/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1072<F: Float>(t1250: F, t43526: F, t7690: F, t96356: F, t3329: F, t8060: F, t3668: F, t8104: F, t15573: F, t28136: F, t27077: F, t26975: F, t993: F, t1856: F, t330: F, t3530: F, t417: F) -> (F, F, F, F, F, F, F, F, F) {
    let t96522 = t43526 * t1250;
    let t96534 = t7690 * t96356;
    let t96543 = t8060 * t3329;
    let t96670 = t8104 * t3668;
    let t96727 = t15573 * t28136;
    let t96728 = t27077 * t96727;
    let t96735 = t993 * t26975;
    let t96736 = t1856 * t330;
    let t96742 = t417 * t3530;
    (t96522, t96534, t96543, t96670, t96727, t96728, t96735, t96736, t96742)
}
