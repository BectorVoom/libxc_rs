//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 578/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk578<F: Float>(t4856: F, t608: F, t1724: F, t1725: F, t606: F, t609: F, t1709: F, t4834: F, t4838: F, t4842: F, t4845: F, t4848: F) -> (F, F, F, F, F, F, F, F) {
    let t4857 = F::new(1.0) / t4856;
    let t4858 = t608 * t4857;
    let t4859 = t1724 * t1724;
    let t4860 = t4859 * t1725;
    let t4864 = F::new(1.0) / t609 / t606;
    let t4865 = t1709 * t1709;
    let t4866 = t4864 * t4865;
    let t4868 = F::new(4.0) / F::new(9.0) * t4834;
    let t4873 = t4868 + F::new(2.0) / F::new(9.0) * t4838 - F::new(2.0) / F::new(9.0) * t4842 + F::new(2.0) / F::new(3.0) * t4845 - t4848 / F::new(3.0);
    (t4857, t4858, t4859, t4860, t4864, t4865, t4866, t4873)
}
