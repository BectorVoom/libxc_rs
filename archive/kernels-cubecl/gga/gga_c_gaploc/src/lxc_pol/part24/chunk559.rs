//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 559/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk559<F: Float>(t1901: F, t2950: F, t2949: F, t550: F, t1843: F, t1022: F, t835: F) -> (F, F, F, F) {
    let t2951 = t1901 * t2950;
    let t2954 = t550 * t2949;
    let t2955 = t1843 * t2954;
    let t2958 = t835 * t1022;
    (t2951, t2954, t2955, t2958)
}
