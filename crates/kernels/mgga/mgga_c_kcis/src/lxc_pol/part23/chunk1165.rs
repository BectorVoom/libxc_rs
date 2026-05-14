//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1165/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1165<F: Float>(t18221: F, t28843: F, t7978: F, t28793: F, t7974: F, t98887: F, t27556: F, t27607: F, t28811: F, t28816: F, t7968: F, t98869: F, t98872: F, t98883: F, t99079: F, t99635: F) -> (F,) {
    let t99639 = t7978 * t18221 * t28843;
    let t99644 = 0.61782407407407407408e-3 * t28793 * t7974;
    let t99646 = 0.23214722222222222222e-2 * t98887;
    let t99655 = -0.92754700520833333334e-4 * t7968 * t99635 - 0.54059606481481481482e-3 * t99639 - 0.17411041666666666666e-2 * t98869 - 0.23214722222222222222e-2 * t98872 + t99644 - 0.34822083333333333332e-2 * t98883 + t99646 - 0.13901041666666666667e-2 * t27607 * t28811 - 0.69505208333333333334e-3 * t27607 * t28816 - 0.92754700520833333334e-4 * t27556 * t28816 + 0.51015085286458333333e-3 * t7968 * t99079;
    (t99655,)
}
