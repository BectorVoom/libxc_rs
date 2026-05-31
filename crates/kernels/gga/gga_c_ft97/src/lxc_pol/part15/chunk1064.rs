//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1064/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1064<F: Float>(t1555: F, t1964: F, t85451: F, t89: F, t20714: F, t925: F, t446: F, t9073: F, t20758: F, t2983: F, t9049: F, t40599: F, t61462: F, t62134: F, t86608: F, t86937: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F) -> (F, F, F, F, F, F) {
    let t86970 = t89 * t1555 * t1964 * t85451;
    let t86973 = t925 * t20714;
    let t86975 = t446 * t9073 * t86973;
    let t86977 = t2983 * t20758;
    let t86979 = t446 * t9049 * t86977;
    let t86981 = -F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t86608 + t86937 / F::cast_from(6.0_f64) + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t61462 + t40599 - F::cast_from(12.0_f64) * t86942 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86946 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t86950 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86954 - t86958 / F::cast_from(9.0_f64) + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86962 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86966 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t86970 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t62134 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86975 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86979;
    (t86970, t86973, t86975, t86977, t86979, t86981)
}
