//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1013/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1013<F: Float>(t11880: F, t11882: F, t1010: F, t3366: F, t1276: F, t1070: F, t2391: F, t11032: F, t11034: F, t11045: F, t11051: F, t11058: F, t11866: F, t11868: F, t11870: F, t11872: F, t11874: F, t11876: F, t11878: F) -> (F, F, F) {
    let t11883 = t11880 * t11882;
    let t11885 = t3366 * t1010;
    let t11886 = t1276 * t11885;
    let t11888 = t1070 * t2391;
    let t11889 = t1276 * t11888;
    let t11893 = -t11032 - t11034 / F::new(3.0) - t11866 / F::new(3.0) - t11868 / F::new(4.0) + t11870 / F::new(8.0) - t11872 / F::new(8.0) + t11874 / F::new(4.0) + t11876 / F::new(3.0) + t11878 / F::new(4.0) - F::new(3.0) / F::new(4.0) * t11883 - F::new(2.0) / F::new(3.0) * t11886 + t11889 / F::new(4.0) + t11045 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t11051 - t11058;
    (t11885, t11888, t11893)
}
