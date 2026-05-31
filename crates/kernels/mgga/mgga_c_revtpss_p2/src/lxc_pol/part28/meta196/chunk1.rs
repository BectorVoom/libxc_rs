//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 960/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk960<F: Float>(t4308: F, t706: F, t1531: F, t705: F, t707: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F, t4307: F) -> (F, F, F, F) {
    let t4310 = F::cast_from(4.0_f64) * t706 * t4308;
    let t4311 = t705 * t1531;
    let t4313 = F::cast_from(4.0_f64) * t4311 * t707;
    let t4314 = t4300 - t2569 + t2579 + t2587 - t2522 - t2498 - t2518 - t4301 + t2526 + t2610 - t4304 - t2562 + t4307 + t4310 + t4313;
    (t4310, t4311, t4313, t4314)
}
