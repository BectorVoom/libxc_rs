//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 545/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk545<F: Float>(t1457: F, t2788: F, t1445: F, t2779: F, t2787: F, t447: F, t528: F, t999: F, t1: F, t986: F) -> (F, F, F, F, F, F) {
    let t2862 = t1457 * t2788;
    let t2865 = t1445 * t2779;
    let t2868 = t2787 * t447;
    let t2869 = t1445 * t2868;
    let t2872 = t528 * t999;
    let t2875 = t986 * t1;
    (t2862, t2865, t2868, t2869, t2872, t2875)
}
