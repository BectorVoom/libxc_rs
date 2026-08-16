//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 866/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk866<F: Float>(t1392: F, t2958: F, t1391: F, t701: F, t8469: F, t1445: F, t1835: F, t8549: F, t1865: F, t1022: F, t5750: F, t3009: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8579 = t1392 * t2958;
    let t8580 = t1391 * t8579;
    let t8587 = t8469 * t701;
    let t8588 = t1445 * t8587;
    let t8591 = t2958 * t1835;
    let t8592 = t1445 * t8591;
    let t8595 = t1445 * t8549;
    let t8600 = t2958 * t1865;
    let t8601 = t1445 * t8600;
    let t8604 = t5750 * t1022;
    let t8605 = t8604 * t1865;
    let t8606 = t1445 * t8605;
    let t8612 = t3009 * t1865;
    (t8580, t8588, t8592, t8595, t8600, t8601, t8604, t8606, t8612)
}
