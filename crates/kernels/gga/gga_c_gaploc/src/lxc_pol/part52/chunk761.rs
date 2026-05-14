//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 761/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk761<F: Float>(t13506: F, t2087: F, t4614: F, t43715: F, t10931: F, t23220: F, t45316: F, t36632: F, t959: F, t2660: F, t36512: F, t10867: F, t10972: F, t1457: F, t2684: F, t45423: F, t7585: F) -> (F, F, F, F, F, F, F) {
    let t45563 = 0.82820720060468819301e2 * t2087 * t4614 * t13506;
    let t45565 = 0.23833659967900284446e0 * t43715;
    let t45569 = 0.27606906686822939767e2 * t23220 * t10931 * t45316;
    let t45574 = t36632 * t959;
    let t45575 = 0.14896037479937677779e-1 * t45574;
    let t45577 = 0.25025342966295298669e1 * t36512 * t2660;
    let t45580 = 0.50050685932590597338e1 * t10867 * t1457 * t10972;
    let t45586 = 0.43710935587469654631e2 * t2684 * t7585 * t45423;
    (t45563, t45565, t45569, t45575, t45577, t45580, t45586)
}
