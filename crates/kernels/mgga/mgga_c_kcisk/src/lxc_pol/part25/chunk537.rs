//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 537/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk537<F: Float>(t1725: F, t4859: F, t606: F, t609: F, t1709: F, t4834: F, t4838: F, t4842: F, t4845: F, t4848: F, t1707: F, t1714: F, t353: F, t579: F, t964: F, t163: F, t657: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4860 = t4859 * t1725;
    let t4864 = 1.0 / t609 / t606;
    let t4865 = t1709 * t1709;
    let t4866 = t4864 * t4865;
    let t4868 = 4.0 / 9.0 * t4834;
    let t4873 = t4868 + 2.0 / 9.0 * t4838 - 2.0 / 9.0 * t4842 + 2.0 / 3.0 * t4845 - t4848 / 3.0;
    let t4874 = t1707 * t4873;
    let t4876 = 0.39862222222222222223e0 * t4834;
    let t4881 = 1.0/f64::sqrt(t606);
    let t4882 = t4881 * t4865;
    let t4884 = t1714 * t4873;
    let t4887 = t353 * t964 * t579;
    let t4888 = 0.27385555555555555555e0 * t4887;
    let t4889 = t163 * t657;
    (t4860, t4864, t4865, t4866, t4868, t4873, t4874, t4876, t4881, t4882, t4884, t4887, t4888, t4889)
}
