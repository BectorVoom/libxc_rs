//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2005/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2005<F: Float>(t116: F, t30552: F, t1940: F, t2255: F, t8020: F, t105928: F, t28472: F, t105902: F, t105909: F, t106510: F, t18280: F, t2071: F, t2403: F, t27169: F, t27402: F, t28456: F, t28460: F, t29591: F, t29602: F, t29606: F, t29713: F, t30420: F, t4541: F, t7010: F, t7428: F, t7432: F, t7749: F, t95976: F) -> (F, F, F, F) {
    let t110110 = t30552 * t116;
    let t110150 = F::new(2.0) * t1940 * t8020 * t2255;
    let t110154 = F::new(2.0) * t28472 * t105928;
    let t110158 = -t1940 * t7432 * t106510 / F::new(2.0) + F::new(3.0) * t2403 * t2071 * t105909 - t1940 * t28460 * t27402 + F::new(3.0) * t4541 * t2071 * t105902 + F::new(3.0) * t2403 * t8020 * t27169 + F::new(3.0) * t2403 * t28456 * t7749 + t1940 * t2071 * t18280 / F::new(2.0) + F::new(3.0) * t2403 * t7428 * t29602 + F::new(3.0) * t4541 * t7428 * t29591 + F::new(3.0) / F::new(2.0) * t2403 * t30420 * t7010 + t110150 + t1940 * t95976 * t29713 - t110154 + F::new(3.0) / F::new(2.0) * t2403 * t7428 * t29606;
    (t110110, t110150, t110154, t110158)
}
