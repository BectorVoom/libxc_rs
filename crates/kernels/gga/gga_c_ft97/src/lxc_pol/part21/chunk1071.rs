//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1071/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1071<F: Float>(t1637: F, t6526: F, t89: F, t1882: F, t26412: F, t26113: F, t487: F, t26269: F, t8392: F, t6488: F, t8232: F, t26343: F, t26180: F, t38953: F, t6535: F, t1326: F, t370: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102743 = t89 * t1637 * t6526;
    let t102751 = 2.0 / 9.0 * t1882 * t26412;
    let t102753 = t487 * t26113;
    let t102759 = 2.0 / 27.0 * t8392 * t26269;
    let t102760 = t8232 * t6488;
    let t102767 = 2.0 / 27.0 * t8392 * t26343;
    let t102772 = 2.0 / 27.0 * t8392 * t26180;
    let t102773 = t38953 * t6535;
    let t102776 = t370 * t1326;
    (t102743, t102751, t102753, t102759, t102760, t102767, t102772, t102773, t102776)
}
