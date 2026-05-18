//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1124/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1124<F: Float>(t1876: F, t914: F, t2169: F, t7673: F, t8024: F, t8122: F, t911: F, t1655: F, t7671: F, t1658: F, t7827: F, t233: F) -> (F, F, F, F, F, F, F) {
    let t27734 = t914 * t1876;
    let t27735 = t2169 * t27734;
    let t27737 = t7673 * t8024;
    let t27739 = t911 * t8122;
    let t27741 = t1655 * t7671;
    let t27743 = t1658 * t7827;
    let t27744 = t233 * t27743;
    (t27734, t27735, t27737, t27739, t27741, t27743, t27744)
}
