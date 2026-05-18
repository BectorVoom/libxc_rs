//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 742/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk742<F: Float>(t5097: F, t696: F, t1806: F, t5105: F, t10449: F, t682: F, t11385: F, t1814: F, t1060: F, t4658: F, t5101: F, t5100: F, t680: F) -> (F, F, F, F, F, F) {
    let t11465 = t696 * t5097;
    let t11467 = t1806 * t5105;
    let t11469 = t682 * t10449;
    let t11472 = t1814 * t11385;
    let t11476 = t5101 * t1060 * t4658;
    let t11480 = F::new(1.0) / t5100 / t680;
    (t11465, t11467, t11469, t11472, t11476, t11480)
}
