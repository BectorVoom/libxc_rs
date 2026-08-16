//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 671/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk671<F: Float>(t1133: F, t3338: F, t7754: F, t2825: F, t389: F, t1096: F, t1189: F, t1021: F, t1196: F, t1200: F, t7746: F, t7750: F, t7752: F) -> (F, F, F, F, F, F, F) {
    let t7755 = t3338 * t1133;
    let t7756 = t7754 * t7755;
    let t7758 = t2825 * t389;
    let t7760 = t1096 * t1189;
    let t7762 = t1021 * t1196;
    let t7764 = t1021 * t1200;
    let t7766 = t7746 / F::cast_from(16.0_f64) - t7750 / F::cast_from(16.0_f64) - t7752 / F::cast_from(6.0_f64) + t7756 / F::cast_from(24.0_f64) - t7758 / F::cast_from(128.0_f64) + t7760 / F::cast_from(128.0_f64) + t7762 / F::cast_from(24.0_f64) - t7764 / F::cast_from(96.0_f64);
    (t7755, t7756, t7758, t7760, t7762, t7764, t7766)
}
