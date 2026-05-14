//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 166/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk166<F: Float>(t571: F, t574: F, t577: F, t581: F) -> (F, F, F) {
    let t760 = 0.705945e1 * t574 + 0.1549425e1 * t571 + 0.420775e0 * t577 + 0.1562925e0 * t581;
    let t763 = 1.0 + 0.32164683177870697974e2 / t760;
    let t764 = f64::ln(t763);
    (t760, t763, t764)
}
