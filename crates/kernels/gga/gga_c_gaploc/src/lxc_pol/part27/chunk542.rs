//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 542/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk542<F: Float>(t2954: F, t549: F, t1048: F, t1628: F, t1029: F, t1589: F, t1043: F, t590: F, t1022: F, t1890: F) -> (F, F, F, F, F, F) {
    let t2979 = t549 * t2954;
    let t2989 = t1628 * t1048;
    let t2992 = t1589 * t1029;
    let t2995 = t1628 * t1043;
    let t3002 = t1029 * t590;
    let t3005 = t1890 * t1022;
    (t2979, t2989, t2992, t2995, t3002, t3005)
}
