//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1064/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1064<F: Float>(t1984: F, t9804: F, t5501: F, t935: F, t2530: F, t321: F, t5580: F, t7802: F, t7809: F, t2012: F, t7426: F, t1423: F, t2554: F) -> (F, F, F, F, F, F, F) {
    let t23000 = t1984 * t9804;
    let t23021 = t5501 * t935;
    let t23092 = t321 * t2530;
    let t23099 = t5580 * t7802;
    let t23104 = t5580 * t7809;
    let t23157 = t2012 * t7426;
    let t23176 = t1423 * t2554;
    (t23000, t23021, t23092, t23099, t23104, t23157, t23176)
}
