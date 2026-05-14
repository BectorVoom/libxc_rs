//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 283/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk283<F: Float>(t1489: F, t556: F, t572: F, t1533: F, t1494: F, t1497: F, t571: F, t1457: F, t552: F, t577: F) -> (F, F, F, F, F, F, F, F) {
    let t1534 = t556 * t1489;
    let t1535 = t572 * t1534;
    let t1536 = t1533 * t1535;
    let t1538 = t1494 * t1497;
    let t1539 = t572 * t1538;
    let t1540 = t571 * t1539;
    let t1542 = t1457 * t552;
    let t1543 = t1542 * t577;
    (t1534, t1535, t1536, t1538, t1539, t1540, t1542, t1543)
}
