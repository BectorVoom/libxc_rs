//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1332/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1332<F: Float>(t27114: F, t901: F, t30843: F, t10396: F, t20565: F, t31586: F, t4820: F, t6824: F, t31591: F, t10399: F, t21272: F, t2478: F, t2792: F, t6576: F) -> (F, F, F, F, F, F, F) {
    let t34700 = t27114 * t901;
    let t34701 = F::cast_from(0.14896037479937677779e-1_f64) * t34700;
    let t34702 = F::cast_from(0.63904876589867916128e-1_f64) * t30843;
    let t34706 = F::cast_from(0.15889106645266856297e0_f64) * t20565 * t10396;
    let t34709 = F::cast_from(0.15889106645266856297e0_f64) * t6824 * t4820 * t31586;
    let t34712 = F::cast_from(0.15889106645266856297e0_f64) * t6824 * t4820 * t31591;
    let t34713 = t21272 * t10399;
    let t34714 = F::cast_from(0.38342925953920749676e0_f64) * t34713;
    let t34716 = t6576 * t2792 * t2478;
    (t34701, t34702, t34706, t34709, t34712, t34714, t34716)
}
