//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 359/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk359<F: Float>(t1814: F, t1824: F, t1806: F, t429: F, t446: F, t686: F, t41: F, t569: F) -> (F, F, F, F) {
    let t1825 = t1814 * t1824;
    let t1829 = 0.11955719325063177623e-1 * t1806;
    let t1834 = 0.3513e-2 * t429 * t446 * t686;
    let t1835 = t41 * t569;
    (t1825, t1829, t1834, t1835)
}
