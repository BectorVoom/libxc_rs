//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1133/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1133<F: Float>(t14546: F, t26685: F, t26748: F, t27816: F, t27822: F, t2894: F, t44684: F, t44743: F, t7703: F, t7704: F, t95621: F, t95686: F, t95688: F, t95691: F, t95696: F, t95698: F, t95713: F) -> (F,) {
    let t95718 = -t95686 - 0.66327777777777777776e-2 * t95688 + 0.37101880208333333334e-3 * t26685 * t95691 + t95696 + t95698 + 0.46336805555555555556e-3 * t26748 * t27816 + 0.23168402777777777778e-3 * t7703 * t2894 * t7704 * t44743 + 0.92673611111111111112e-3 * t7703 * t14546 * t7704 * t44684 + 0.46336805555555555556e-3 * t26748 * t27822 + 0.23168402777777777778e-3 * t7703 * t95713 - 0.92754700520833333335e-4 * t26685 * t95621;
    (t95718,)
}
