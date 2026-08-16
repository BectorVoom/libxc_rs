//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1194/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1194<F: Float>(t112741: F, t112743: F, t113053: F, t118672: F, t118729: F, t118768: F, t13053: F, t13065: F, t1528: F, t25348: F, t2597: F, t2713: F, t2718: F, t30630: F, t30647: F, t30729: F, t32796: F, t4147: F, t4268: F, t6632: F, t6662: F, t6663: F, t7537: F, t8353: F, t855: F, t858: F) -> F {
    let t118791 = F::cast_from(0.82246703342411321825e-2_f64) * t112741;
    let t118792 = F::cast_from(0.76763589786250567036e-1_f64) * t112743;
    let t118793 = -t113053 * t1528 - t118672 + F::cast_from(4.0_f64) * t855 * t2718 * t6662 * t7537 - t855 * t858 * (t118729 + t118768) + F::cast_from(4.0_f64) * t4147 * t30630 - F::cast_from(6.0_f64) * t2713 * t32796 - F::cast_from(2.0_f64) * t25348 * t6663 - t4268 * t30729 + F::cast_from(4.0_f64) * t25348 * t6632 - F::cast_from(6.0_f64) * t2597 * t32796 + F::cast_from(2.0_f64) * t4147 * t30647 + F::cast_from(2.0_f64) * t13053 * t8353 + F::cast_from(2.0_f64) * t13065 * t8353 + F::cast_from(4.0_f64) * t4268 * t30630 + t118791 + t118792;
    t118793
}
