//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1182/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1182<F: Float>(t12292: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t1132: F) -> (F, F) {
    let t12322 = -t12296 + F::new(4.0) / F::new(9.0) * t12297 + F::new(2.0) / F::new(9.0) * t12299 - F::new(2.0) / F::new(3.0) * t12301 - t12303 / F::new(3.0) + F::new(10.0) / F::new(27.0) * t12307 - F::new(4.0) / F::new(3.0) * t12310 - F::new(2.0) / F::new(3.0) * t12292 + F::new(2.0) * t12314 + F::new(2.0) * t12317 + t12320 / F::new(3.0);
    let t12323 = t1132 * t12322;
    (t12322, t12323)
}
