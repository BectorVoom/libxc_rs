//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1000/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1000<F: Float>(t508: F, t5920: F, t4303: F, t4306: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2628: F, t2632: F) -> (F, F, F, F) {
    let t5921 = t508 * t5920;
    let t5924 = F::cast_from(0.36622894612013090108e-3_f64) * t4303;
    let t5925 = F::new(8.0) * t4306;
    let t5926 = -t2569 + t2579 + t2587 - t2522 - t2498 - t2518 + t2610 - t5924 - t2562 + t5925 + t2632 + t2628;
    (t5921, t5924, t5925, t5926)
}
