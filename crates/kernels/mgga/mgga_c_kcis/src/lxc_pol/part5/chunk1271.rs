//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1271/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1271<F: Float>(t16906: F, t21134: F, t25: F, t7081: F, t493: F, t1930: F, t5718: F, t1368: F, t1382: F, t16850: F, t21103: F, t21107: F, t21111: F, t21117: F, t21121: F, t21126: F, t21131: F, t5691: F, t5723: F, t5734: F, t7054: F) -> F {
    let t21135 = t16906 * t21134;
    let t21138 = t25 * t7081;
    let t21139 = t493 * t21138;
    let t21141 = t1930 * t5718;
    let t21148 = t1368 * t21103 / F::cast_from(144.0_f64) + t1368 * t21107 / F::cast_from(48.0_f64) + t1368 * t21111 / F::cast_from(36.0_f64) + t5691 * t5723 / F::cast_from(54.0_f64) - t1368 * t21117 / F::cast_from(144.0_f64) - t1368 * t21121 / F::cast_from(216.0_f64) - t1368 * t21126 / F::cast_from(36.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1368 * t21131 - t1368 * t21135 / F::cast_from(54.0_f64) + t21139 / F::cast_from(144.0_f64) + t21141 / F::cast_from(54.0_f64) + t1930 * t5734 / F::cast_from(18.0_f64) - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t7054 * t1382 - t16850 / F::cast_from(216.0_f64);
    t21148
}
