//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1101/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1101<F: Float>(t218: F, t219: F, t9832: F, t6175: F, t6177: F, t7950: F, t7980: F, t7983: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F, t9830: F, t9810: F, t852: F) -> (F, F, F) {
    let t9834 = t218 * t219 * t9832;
    let t9836 = 0.15358125e0 * t9812 + 0.3071625e0 * t9814 - t6175 + 0.27385555555555555556e0 * t6177 + 0.5477111111111111111e0 * t7950 - t7980 - t7983 - 0.16431333333333333333e0 * t9819 + 0.49294e0 * t9823 - 0.16431333333333333333e0 * t9826 + 0.24647e0 * t9830 + 0.24647e0 * t9834;
    let t9837 = t9810 + t9836;
    let t9838 = t9837 * t852;
    (t9834, t9837, t9838)
}
