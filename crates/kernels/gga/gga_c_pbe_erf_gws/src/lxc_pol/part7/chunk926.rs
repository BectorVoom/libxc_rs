//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 926/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk926<F: Float>(t4560: F, t461: F, t409: F, t4743: F, t31: F, t4: F, t4573: F, t1392: F, t4793: F, t472: F, t4778: F, t75: F, t1399: F, t4835: F, t1318: F, t1216: F) -> (F, F, F, F, F, F, F, F) {
    let t18623 = t4560 * t461;
    let t18624 = 576.0 * t18623;
    let t18625 = t409 * t4743;
    let t18626 = 16.0 * t18625;
    let t18629 = 0.11483710345679012345e-1 * t4 * t4573 * t31;
    let t18630 = t4793 * t1392;
    let t18631 = 0.1038945353962551798e3 * t18630;
    let t18633 = t4778 * t75 * t472;
    let t18634 = 0.23392893589820816284e1 * t18633;
    let t18635 = t1399 * t4835;
    let t18636 = 0.14035736153892489771e2 * t18635;
    let t18637 = t1318 * t1318;
    let t18638 = 1.0 / t18637;
    let t18639 = t1216 * t1216;
    (t18624, t18626, t18629, t18631, t18634, t18636, t18638, t18639)
}
