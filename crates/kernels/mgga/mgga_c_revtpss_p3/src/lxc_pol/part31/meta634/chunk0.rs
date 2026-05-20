//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2088/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088<F: Float>(t15749: F, t7117: F, t25490: F, t4845: F, t15666: F, t27479: F, t3215: F, t25577: F, t4817: F, t15711: F, t7132: F, t15655: F, t1972: F) -> (F, F, F, F, F, F, F) {
    let t100329 = t7117 * t15749;
    let t100332 = F::cast_from(0.57165357490759649296e-3_f64) * t25490 * t4845;
    let t100334 = F::cast_from(0.57165357490759649296e-3_f64) * t7117 * t15666;
    let t100336 = F::cast_from(0.57165357490759649296e-3_f64) * t27479 * t3215;
    let t100342 = F::cast_from(0.20325460441158986416e-2_f64) * t25577 * t4817;
    let t100343 = t7132 * t15711;
    let t100345 = t15655 * t1972;
    (t100329, t100332, t100334, t100336, t100342, t100343, t100345)
}
