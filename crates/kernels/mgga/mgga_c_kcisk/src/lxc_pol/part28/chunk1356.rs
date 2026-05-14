//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1356/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1356<F: Float>(t34045: F, t34122: F, t17182: F, t35211: F, t9664: F, t1757: F, t1869: F, t34159: F, t71037: F, t112283: F, t116120: F, t116289: F, t116321: F, t116482: F, t116513: F, t116965: F, t1636: F, t1791: F, t33031: F, t34013: F, t34023: F, t34032: F, t34039: F, t34182: F, t34192: F, t5015: F, t8820: F, t9940: F) -> (F, F) {
    let t121184 = t34122 * t34045;
    let t121203 = t9664 * t17182 * t35211;
    let t121214 = t1869 * t34159 * t71037 * t1757;
    let t121216 = 0.69444444444444444447e-2 * t121184 + 0.20833333333333333334e-1 * t116120 * t9940 + t116289 - 0.20833333333333333334e-1 * t34122 * t34182 - 0.80416666666666666669e-2 * t34192 * t34182 + 0.34722222222222222223e-2 * t33031 * t5015 * t1791 * t8820 * t1636 - 0.18518518518518518519e-1 * t116482 * t34023 + 0.23148148148148148149e-2 * t116321 + 0.26805555555555555556e-2 * t116965 * t34013 + 0.34722222222222222223e-2 * t121203 - 0.23148148148148148149e-2 * t112283 + 0.69444444444444444446e-2 * t116513 * t34032 + 0.69444444444444444446e-2 * t116513 * t34013 + 0.13888888888888888889e-1 * t116513 * t34039 + 0.33163888888888888888e-2 * t121214;
    (t121214, t121216)
}
