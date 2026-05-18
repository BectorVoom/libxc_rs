//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 923/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk923<F: Float>(t1645: F, t2586: F, t3307: F, t9420: F, t813: F, t3280: F, t549: F, t2033: F, t325: F, t40: F) -> (F, F, F, F, F, F) {
    let t9972 = t1645 * t2586;
    let t9981 = t9420 * t3307;
    let t9982 = t813 * t9981;
    let t10004 = t549 * t3280;
    let t10006 = F::new(0.59584149919750711116e-1) * t2033 * t10004;
    let t10007 = t40 * t325;
    (t9972, t9981, t9982, t10004, t10006, t10007)
}
