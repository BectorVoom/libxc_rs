//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 891/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk891<F: Float>(t16906: F, t21134: F, t25: F, t7081: F, t493: F, t1930: F, t5718: F, t1368: F, t1382: F, t16850: F, t21103: F, t21107: F, t21111: F, t21117: F, t21121: F, t21126: F, t21131: F, t5691: F, t5723: F, t5734: F, t7054: F) -> F {
    let t21135 = t16906 * t21134;
    let t21138 = t25 * t7081;
    let t21139 = t493 * t21138;
    let t21141 = t1930 * t5718;
    let t21148 = t1368 * t21103 / F::new(144.0) + t1368 * t21107 / F::new(48.0) + t1368 * t21111 / F::new(36.0) + t5691 * t5723 / F::new(54.0) - t1368 * t21117 / F::new(144.0) - t1368 * t21121 / F::new(216.0) - t1368 * t21126 / F::new(36.0) + F::new(7.0) / F::new(648.0) * t1368 * t21131 - t1368 * t21135 / F::new(54.0) + t21139 / F::new(144.0) + t21141 / F::new(54.0) + t1930 * t5734 / F::new(18.0) - F::new(11.0) / F::new(108.0) * t7054 * t1382 - t16850 / F::new(216.0);
    t21148
}
