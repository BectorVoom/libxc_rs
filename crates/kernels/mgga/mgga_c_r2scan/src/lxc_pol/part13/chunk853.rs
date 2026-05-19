//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 853/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk853<F: Float>(t2755: F, t410: F, t1823: F, t963: F, t2747: F, t741: F, t1827: F, t1693: F, t898: F, t2483: F, t697: F, t5344: F, t5346: F, t5350: F, t5354: F, t5355: F, t5360: F) -> F {
    let t7720 = F::new(8.0) * t410 * t2755;
    let t7721 = t963 * t1823;
    let t7724 = F::cast_from(0.23392894490538584828e1_f64) * t2747 * t741;
    let t7725 = t963 * t1827;
    let t7727 = t898 * t1693;
    let t7730 = F::cast_from(0.1301229756036208781e0_f64) * t2483 * t697;
    let t7731 = F::cast_from(0.70178683471615754484e1_f64) * t5344 - F::cast_from(0.20779030926817756511e3_f64) * t5346 - t5350 - t5354 - F::cast_from(0.23392894490538584828e1_f64) * t5355 - t5360 + t7720 + F::cast_from(0.10254018858216406658e4_f64) * t7721 - t7724 - F::cast_from(0.23392894490538584828e1_f64) * t7725 - F::cast_from(0.1301229756036208781e0_f64) * t7727 + t7730;
    t7731
}
