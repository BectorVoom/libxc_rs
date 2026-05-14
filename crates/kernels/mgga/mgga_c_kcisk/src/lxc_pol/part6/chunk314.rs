//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 314/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk314<F: Float>(t1646: F, t677: F, t1634: F, t1806: F, t429: F, t446: F, t686: F, t41: F, t569: F) -> (F, F, F, F, F) {
    let t1815 = t1646 * t677;
    let t1819 = 0.41275e-2 * t1634;
    let t1829 = 0.11955719325063177623e-1 * t1806;
    let t1834 = 0.3513e-2 * t429 * t446 * t686;
    let t1835 = t41 * t569;
    (t1815, t1819, t1829, t1834, t1835)
}
