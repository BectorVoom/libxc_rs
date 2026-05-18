//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 948/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk948<F: Float>(t1619: F, t309: F, t33743: F, t620: F, t2131: F, t2147: F, t2341: F, t847: F, t2331: F, t862: F, t865: F, t1219: F, t615: F, t8396: F) -> (F, F, F, F) {
    let t33744 = t1619 * t309;
    let t33747 = F::new(0.10408353825846239354e2) * t33743 * t620 * t33744;
    let t33767 = t2131 * t2147 * t2341 * t847;
    let t33771 = t862 * t2331 * t865;
    let t33778 = t615 * t8396 * t1219;
    (t33747, t33767, t33771, t33778)
}
