//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1236/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1236<F: Float>(t104884: F, t104888: F, t118839: F, t118843: F, t118852: F, t118856: F, t118869: F, t118876: F, t22767: F, t23732: F, t23847: F, t30063: F, t39852: F, t4677: F, t5802: F, t7335: F, t8838: F, t94891: F, t94892: F) -> (F,) {
    let t118879 = 0.24163653553615319119e1 * t5802 * t118852 - 0.10947790369858991997e1 * t7335 * t118856 + 0.10947790369858991997e1 * t94891 * t94892 * t4677 + 0.21895580739717983994e1 * t39852 * t118856 - t104884 - 0.14817333576131687244e-1 * t104888 - 0.10668480174814814815e1 * t23732 * t22767 * t30063 + 0.13335600218518518519e0 * t118869 - 0.45306850413028723348e0 * t23847 * t118843 + 0.45306850413028723348e0 * t8838 * t118839 + 0.48327307107230638238e1 * t23847 * t118876;
    (t118879,)
}
