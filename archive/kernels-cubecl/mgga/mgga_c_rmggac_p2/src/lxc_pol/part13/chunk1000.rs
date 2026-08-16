//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1000/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1000<F: Float>(t1971: F, t3351: F, t5194: F, t880: F, t2144: F, t31043: F, t7720: F, t8592: F, t34847: F, t9046: F, t2186: F, t8587: F) -> (F, F, F, F, F) {
    let t41949 = t3351 * t1971 * t880 * t5194;
    let t41954 = t3351 * t1971 * t2144 * t31043;
    let t41956 = t7720 * t8592;
    let t41958 = t34847 * t9046;
    let t41960 = t2186 * t8587;
    (t41949, t41954, t41956, t41958, t41960)
}
