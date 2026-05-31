//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1298/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1298<F: Float>(t486: F, t21453: F, t1378: F, t286: F, t25: F, t7087: F, t493: F, t1371: F, t18431: F, t1370: F, t12129: F, t12131: F, t1368: F, t1373: F, t16854: F, t16858: F, t16866: F, t21155: F, t21157: F, t21163: F, t21167: F, t5691: F, t5715: F, t5728: F) -> F {
    let t495 = F::cast_from(0.0_f64) < t486;
    let t21455 = piecewise3::<F>(t495, t21453, -t21453);
    let t21456 = t1378 * t21455;
    let t21457 = t286 * t21456;
    let t21460 = t25 * t7087;
    let t21461 = t493 * t21460;
    let t21463 = t1371 * t18431;
    let t21464 = t1370 * t21463;
    let t21467 = t16854 - t16858 / F::cast_from(648.0_f64) - t16866 + t12131 / F::cast_from(432.0_f64) - t5691 * t5728 / F::cast_from(9.0_f64) + t21155 / F::cast_from(648.0_f64) + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t21157 * t1373 + t5691 * t5715 / F::cast_from(27.0_f64) + t21163 / F::cast_from(864.0_f64) + t12129 + t1368 * t21167 / F::cast_from(48.0_f64) - t493 * t21457 / F::cast_from(96.0_f64) - t21461 / F::cast_from(288.0_f64) + t1368 * t21464 / F::cast_from(288.0_f64);
    t21467
}
