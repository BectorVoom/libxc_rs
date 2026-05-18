//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 365/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk365<F: Float>(t1646: F, t677: F, t1634: F, t1638: F, t1649: F, t1648: F, t574: F) -> (F, F, F) {
    let t1815 = t1646 * t677;
    let t1819 = F::new(0.41275e-2) * t1634;
    let t1821 = F::new(0.1982e-1) * t1649 - t1819 - F::new(0.41275e-2) * t1638;
    let t1824 = t1815 * t1648 / F::new(4.0) + t574 * t1821 / F::new(2.0);
    (t1815, t1821, t1824)
}
