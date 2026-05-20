//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2926/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926<F: Float>(t141: F, t2908: F, t77588: F, t77592: F, t77525: F, t77529: F, t63533: F, t63538: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F) -> (F, F, F, F, F) {
    let t77829 = t141 * t2908 * t77588;
    let t77832 = t141 * t2908 * t77592;
    let t77835 = t141 * t2908 * t77525;
    let t77838 = t141 * t2908 * t77529;
    let t77846 = -F::cast_from(0.91983333333333333334e-1_f64) * t63533 + F::new(0.5519e0) * t63538 - F::new(0.99342e0) * t77829 + F::new(0.49671e0) * t77832 - F::new(0.82785e-1) * t77835 - F::new(0.82785e-1) * t77838 - F::new(0.33114e0) * t63541 + F::new(0.5519e-1) * t63543 - F::new(0.27595e0) * t63545 - F::new(0.33114e0) * t63547 + F::new(0.11038e0) * t63549 + F::cast_from(0.73586666666666666666e-1_f64) * t63551;
    (t77829, t77832, t77835, t77838, t77846)
}
