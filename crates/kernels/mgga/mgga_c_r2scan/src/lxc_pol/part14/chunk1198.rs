//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1198/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1198<F: Float>(t10610: F, t10611: F, t12056: F, t10940: F, t12086: F, t11336: F, t2850: F, t3270: F, t3269: F, t3262: F, t3465: F, t40579: F) -> (F, F, F, F) {
    let t41294 = F::new(3.0) / F::new(2.0) * t10610 * t12056 * t10611;
    let t41296 = t10940 * t12086 / F::new(4.0);
    let t41298 = t3270 * t11336 * t2850;
    let t41300 = t3269 * t41298 / F::new(2.0);
    let t41305 = F::new(3.0) / F::new(4.0) * t3262 * t3465 * t40579;
    (t41294, t41296, t41300, t41305)
}
