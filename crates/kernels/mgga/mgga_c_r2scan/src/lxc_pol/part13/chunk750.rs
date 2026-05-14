//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 750/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk750<F: Float>(t2461: F, t424: F, t6801: F, t6881: F, t6885: F, t6888: F, t7117: F, t7121: F, t7126: F, t7128: F, t7129: F, t7132: F, t7133: F, t881: F, t1048: F, t2262: F, t2867: F) -> (F, F, F) {
    let t7136 = t424 * t2461;
    let t7139 = t7117 - t7121 + t6801 + t6881 - 0.4726e1 * t6885 - 0.4726e1 * t6888 - t7126 - t7128 - 0.2363e1 * t7129 - t7132 - 0.2363e1 * t881 * t7133 - 0.4726e1 * t881 * t7136;
    let t7141 = t1048 * t2867 * t2262;
    (t7136, t7139, t7141)
}
