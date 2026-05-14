//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 734/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk734<F: Float>(t1: F, t41801: F, t544: F, t1424: F, t2386: F, t40116: F, t41752: F, t41753: F, t41754: F, t41759: F, t41761: F, t41763: F, t41767: F, t41769: F, t41773: F, t41777: F, t41781: F, t41783: F, t41787: F, t41790: F, t41793: F, t41794: F, t41798: F, t41800: F) -> (F,) {
    let t41803 = t544 * t41801 * t1;
    let t41806 = -t41752 - t41753 + t41754 - 0.85206502119823888169e-1 * t40116 - t41759 + t41761 - 0.10725146985555128001e1 * t41763 * t2386 + t41767 - 0.18404604457881959845e2 * t41769 - t41773 + t41777 + t41781 - t41783 - t41787 - t41790 + t41793 - 0.92023022289409799224e1 * t41794 - 0.92023022289409799224e1 * t41798 + t41800 - 0.39722766613167140743e-1 * t41803 * t1424;
    (t41806,)
}
