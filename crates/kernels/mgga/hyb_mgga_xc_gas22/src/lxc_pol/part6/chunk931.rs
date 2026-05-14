//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 931/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk931<F: Float>(t8688: F, t8691: F, t6597: F, t8670: F, t8673: F, t8676: F, t8683: F, t8685: F, t8695: F, t8699: F, t8703: F, t8706: F, t8840: F, t810: F, t788: F, t3363: F, t820: F) -> (F, F, F, F, F, F) {
    let t8846 = 0.32862666666666666666e0 * t8688;
    let t8847 = 0.32862666666666666666e0 * t8691;
    let t8852 = 0.142419375e1 * t8670 - 0.76790625e-1 * t8673 + 0.39862222222222222223e0 * t8676 + 0.1898925e1 * t8683 + 0.3071625e0 * t8685 - t6597 - t8846 - t8847 + 0.24647e0 * t8695 + 0.49294e0 * t8699 + 0.24647e0 * t8703 + 0.27385555555555555555e0 * t8706;
    let t8853 = t8840 + t8852;
    let t8854 = t8853 * t810;
    let t8856 = 1.0 * t788 * t8854;
    let t8857 = t3363 * t820;
    (t8846, t8847, t8853, t8854, t8856, t8857)
}
