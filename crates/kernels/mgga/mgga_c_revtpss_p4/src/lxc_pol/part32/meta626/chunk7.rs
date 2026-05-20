//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1997/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1997<F: Float>(t101886: F, t108733: F, t108737: F, t108745: F, t108807: F, t108810: F, t108813: F, t2048: F, t26187: F, t28105: F, t28109: F, t28602: F, t29538: F, t29544: F, t29548: F, t7343: F, t7352: F, t7706: F) -> F {
    let t109970 = -F::new(10.0) / F::new(3.0) * t101886 * t7706 - F::new(10.0) / F::new(3.0) * t28602 * t28105 - F::new(10.0) / F::new(3.0) * t28602 * t28109 - F::new(4.0) / F::new(3.0) * t108807 * t2048 - F::new(4.0) / F::new(3.0) * t108810 * t2048 - F::new(4.0) / F::new(3.0) * t108813 * t2048 - F::new(4.0) / F::new(3.0) * t29538 * t7352 - F::new(10.0) / F::new(3.0) * t26187 * t29544 - F::new(10.0) / F::new(3.0) * t7343 * t108733 - F::new(10.0) / F::new(3.0) * t7343 * t108737 - F::new(5.0) / F::new(3.0) * t26187 * t29548 - F::new(5.0) / F::new(3.0) * t7343 * t108745;
    t109970
}
