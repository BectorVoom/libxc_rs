//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1007/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1007<F: Float>(t5: F, t29387: F, t29419: F, t117: F, t1310: F, t1843: F, t2127: F, t27136: F, t27139: F, t27152: F, t27156: F, t27834: F, t27835: F, t28022: F, t28045: F, t28058: F, t28060: F, t508: F, t5517: F, t649: F, t7584: F, t8152: F, t8233: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t29421 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t29387 + t29419);
    let t29422 = t29421 * t117;
    let t29425 = -t1310 * t8152 - t1843 * t7584 - t2127 * t5517 - t29422 * t508 - t649 * t8233 - t27136 - t27139 + t27152 - t27156 + t27834 + t27835 + t28022 - t28045 - t28058 - t28060;
    (t29421, t29422, t29425)
}
