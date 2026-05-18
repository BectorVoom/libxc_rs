//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 455/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk455<F: Float>(t1791: F, t213: F, t661: F, t186: F, t211: F, t582: F, t618: F, t616: F, t196: F, t596: F) -> (F, F, F, F, F, F, F, F) {
    let t1792 = t213 * t1791;
    let t1793 = t661 * t661;
    let t1794 = t1792 * t1793;
    let t1795 = t186 * t1794;
    let t1797 = F::new(4.0) / F::new(15.0) * t211 * t1795;
    let t1798 = t582 * t618;
    let t1799 = t616 * t1798;
    let t1800 = F::new(16.0) / F::new(45.0) * t1799;
    let t1802 = F::new(1.0) / t596 / t196;
    (t1793, t1794, t1795, t1797, t1798, t1799, t1800, t1802)
}
