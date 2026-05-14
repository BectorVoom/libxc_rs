//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 869/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk869<F: Float>(t1381: F, t2627: F, t922: F, t96: F, t2614: F, t3992: F, t1378: F, t4: F, t657: F, t2611: F, t2620: F, t2623: F, t1: F, t283: F, t4027: F, t4047: F, t807: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14856 = t1381 * t2627;
    let t14866 = t96 * t922;
    let t14880 = t3992 * t2614;
    let t14883 = t1378 * t4 * t657;
    let t14885 = t3992 * t2611;
    let t14890 = t3992 * t2620;
    let t14892 = t3992 * t2623;
    let t14898 = t4027 * t1 * t283;
    let t14900 = t4047 * t807;
    (t14856, t14866, t14880, t14883, t14885, t14890, t14892, t14898, t14900)
}
