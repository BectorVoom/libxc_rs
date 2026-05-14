//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 574/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk574<F: Float>(t3601: F, t5750: F, t723: F, t1445: F, t11623: F, t11603: F, t701: F, t1: F, t11656: F, t787: F, t11661: F, t1589: F, t3626: F, t11576: F, t836: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11832 = t5750 * t3601;
    let t11833 = t11832 * t723;
    let t11834 = t1445 * t11833;
    let t11837 = t1445 * t11623;
    let t11840 = t11603 * t701;
    let t11841 = t1445 * t11840;
    let t11844 = t11656 * t1;
    let t11845 = t787 * t11844;
    let t11848 = t11661 * t1;
    let t11849 = t787 * t11848;
    let t11854 = t1589 * t3626;
    let t11861 = t836 * t11576;
    let t11862 = t568 * t11861;
    (t11832, t11834, t11837, t11841, t11844, t11845, t11848, t11849, t11854, t11862)
}
