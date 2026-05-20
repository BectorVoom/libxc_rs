//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1811/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1811<F: Float>(t1923: F, t2048: F, t26175: F, t26207: F, t28154: F, t28598: F, t28600: F, t28602: F, t28628: F, t28638: F, t28641: F, t29513: F, t29538: F, t29544: F, t29548: F, t29551: F, t29554: F, t29562: F, t30543: F, t7343: F, t7702: F, t7706: F, t7709: F, t7964: F) -> F {
    let t30551 = -F::new(10.0) / F::new(3.0) * t28602 * t7706 - F::new(4.0) / F::new(3.0) * t29538 * t2048 - F::new(10.0) / F::new(3.0) * t7343 * t29544 - F::new(5.0) / F::new(3.0) * t7343 * t29548 - F::new(2.0) / F::new(3.0) * t29551 * t2048 - F::new(2.0) / F::new(3.0) * t29554 * t2048 - F::new(4.0) / F::new(3.0) * t7709 * t7964 + F::new(80.0) / F::new(9.0) * t28598 + F::new(32.0) / F::new(9.0) * t28600 - F::new(16.0) / F::new(9.0) * t28638 + t29513 * t2048 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7702 * t7964 + t1923 * t30543 / F::new(3.0) - F::new(16.0) / F::new(9.0) * t28641 + F::new(10.0) * t26175 * t29562 + F::new(20.0) / F::new(3.0) * t28154 * t28628 + t26207;
    t30551
}
