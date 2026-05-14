//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 675/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk675<F: Float>(t2615: F, t2643: F, t3444: F, t582: F, t185: F, t1006: F, t2756: F, t2741: F, t2753: F, t3563: F, t616: F, t3479: F, t636: F, t3493: F, t3443: F, t597: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10851 = t2615 * t2643;
    let t10871 = t582 * t3444;
    let t10872 = t185 * t10871;
    let t10874 = t1006 * t2756;
    let t10876 = t2741 * t2753;
    let t10878 = t582 * t3563;
    let t10879 = t616 * t10878;
    let t10887 = t3479 * t636;
    let t10889 = t3493 * t636;
    let t10908 = t597 * t3443;
    (t10851, t10871, t10872, t10874, t10876, t10878, t10879, t10887, t10889, t10908)
}
