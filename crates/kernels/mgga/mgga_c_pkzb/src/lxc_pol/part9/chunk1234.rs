//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1234/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1234<F: Float>(t1066: F, t154: F, t18060: F, t276: F, t2057: F, t2883: F, t735: F, t7620: F, t1419: F, t17874: F, t17881: F, t17890: F, t2886: F, t2891: F, t7586: F, t7594: F, t7598: F, t7602: F, t7655: F, t7660: F, t7725: F) -> F {
    let t21538 = t276 * t154 * t18060 * t1066;
    let t21540 = t2057 * t2883;
    let t21542 = t735 * t7620;
    let t21543 = t21542 / F::new(54.0);
    let t21559 = -F::new(5.0) / F::new(1296.0) * t21538 - F::new(11.0) / F::new(108.0) * t21540 - t21543 - F::new(5.0) / F::new(162.0) * t17874 - t17881 + F::new(11.0) / F::new(18.0) * t1419 * t2886 * t2891 + t7586 * t7602 / F::new(2.0) - t7586 * t7594 / F::new(3.0) - t7586 * t7598 / F::new(6.0) + F::cast_from(0.68598428988911579154e-2_f64) * t7725 * t7655 + F::cast_from(0.34299214494455789577e-2_f64) * t7725 * t7660 + t17890 / F::new(48.0);
    t21559
}
