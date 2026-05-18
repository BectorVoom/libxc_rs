//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 912/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk912<F: Float>(t2271: F, t3165: F, t6881: F, t6888: F, t7126: F, t7129: F, t7132: F, t8649: F, t8651: F, t8652: F, t881: F, t9056: F, t9787: F, t9791: F) -> F {
    let t9816 = t2271 * t3165;
    let t9818 = -F::new(0.2363e1) * t881 * t9056 - t9787 - t8649 + t8651 + t6881 - t9791 - F::new(0.2363e1) * t6888 - t7126 - t8652 - F::new(0.4726e1) * t7129 - t7132 - F::new(0.4726e1) * t9816;
    t9818
}
