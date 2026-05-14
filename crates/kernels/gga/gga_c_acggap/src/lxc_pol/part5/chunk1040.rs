//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1040/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1040<F: Float>(t12838: F, t12840: F, t12842: F, t12844: F, t12848: F, t15826: F, t15828: F, t15830: F, t15832: F, t15841: F, t15849: F, t20857: F, t1165: F, t3194: F, t5284: F, t5862: F) -> (F, F) {
    let t20870 = 0.32012600194825403606e-1 * t20857 - 0.34299214494455789578e-2 * t15826 - 0.24009450146119052704e-1 * t15828 - 0.16006300097412701803e-1 * t15830 - 0.12004725073059526352e-1 * t15832 + 0.34299214494455789578e-2 * t12838 - 0.25724410870841842183e-2 * t12840 - 0.17149607247227894789e-2 * t12842 + 0.25724410870841842183e-2 * t12844 - 0.80031500487063509016e-2 * t12848 + 0.17149607247227894789e-1 * t15841 - 0.68026775414003982663e-1 * t15849;
    let t20875 = t3194 * t1165 * t5862 * t5284;
    (t20870, t20875)
}
