//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2901/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901<F: Float>(t3011: F, t4682: F, t11506: F, t1626: F, t1609: F, t2924: F, t11112: F, t2875: F, t4632: F, t11294: F, t15098: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51878: F, t51881: F, t51884: F, t51887: F) -> (F, F, F, F, F, F) {
    let t52637 = t4682 * t3011;
    let t52642 = t1626 * t11506;
    let t52645 = t2924 * t1609;
    let t52647 = F::new(18.0) * t52645 * t11112;
    let t52650 = F::new(18.0) * t2924 * t4632 * t2875;
    let t52652 = F::new(18.0) * t11294 * t15098;
    let t52664 = F::new(0.71752e1) * t51849 - F::cast_from(0.19931111111111111111e0_f64) * t51853 - F::cast_from(0.88582716049382716048e0_f64) * t51858 + F::new(0.17938e1) * t51863 + F::new(0.17938e1) * t51867 + F::cast_from(0.59793333333333333334e0_f64) * t51871 - F::cast_from(0.71752000000000000002e1_f64) * t51875 + F::cast_from(0.427258125e1_f64) * t51878 - F::cast_from(0.230371875e0_f64) * t51881 + F::new(0.46074375e0) * t51884 - F::new(0.28483875e1) * t51887;
    (t52637, t52642, t52647, t52650, t52652, t52664)
}
