//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1015/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1015<F: Float>(t15629: F, t504: F, t69870: F, t71516: F, t75572: F, t75590: F, t75593: F, t75596: F, t75602: F, t77664: F, t77665: F, t77666: F, t77669: F, t77670: F, t77672: F, t77677: F, t77679: F, t77681: F) -> F {
    let t77682 = -F::cast_from(0.2363e1_f64) * t71516 - F::cast_from(0.15372131649401827111e-4_f64) * t75572 - t77664 - t77665 + t77666 - F::cast_from(0.19957069503106347607e-1_f64) * t504 * t15629 + t77669 - t77670 - t77672 + F::cast_from(0.17347588262831798123e-4_f64) * t75590 + F::cast_from(0.17347588262831798123e-4_f64) * t75593 + F::cast_from(0.12263514265030957031e-4_f64) * t69870 - F::cast_from(0.81756761766873046877e-6_f64) * t75596 + t77677 + t75602 - t77679 - t77681;
    t77682
}
