//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1072/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1072<F: Float>(t12939: F, t1625: F, t209: F, t736: F, t4188: F, t5895: F, t12344: F, t2016: F, t2118: F, t3751: F, t4992: F, t86: F, t3960: F, t5623: F, t1494: F, t5627: F) -> (F, F, F, F, F, F, F, F) {
    let t40662 = t1625 * t12939;
    let t44682 = t209 * t736;
    let t48044 = t5895 * t4188;
    let t48058 = t2016 * t12344;
    let t51097 = t2118 * t12939;
    let t51692 = t86 * t4992 * t3751;
    let t51799 = t5623 * t3960;
    let t52073 = t1494 * t5627;
    (t40662, t44682, t48044, t48058, t51097, t51692, t51799, t52073)
}
