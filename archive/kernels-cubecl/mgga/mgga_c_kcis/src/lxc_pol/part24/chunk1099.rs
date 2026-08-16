//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1099/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1099<F: Float>(t29010: F, t4939: F, t2173: F, t26685: F, t26856: F, t27832: F, t27967: F, t27969: F, t27975: F, t27981: F, t28988: F, t28997: F, t29001: F, t29004: F, t29007: F, t7703: F, t8038: F) -> (F, F) {
    let t29011 = t4939 * t29010;
    let t29022 = t26856 - F::cast_from(0.13901041666666666667e-2_f64) * t2173 * t28997 - F::cast_from(0.55273148148148148147e-3_f64) * t29001 + F::cast_from(0.49745833333333333332e-2_f64) * t29004 - F::cast_from(0.23168402777777777778e-3_f64) * t7703 * t29007 - F::cast_from(0.30891203703703703704e-3_f64) * t7703 * t29011 - F::cast_from(0.18550940104166666667e-3_f64) * t26685 * t28988 - F::cast_from(0.46336805555555555556e-3_f64) * t27832 * t8038 - F::cast_from(0.46336805555555555556e-3_f64) * t27967 - F::cast_from(0.33163888888888888888e-2_f64) * t27969 + F::cast_from(0.46336805555555555556e-3_f64) * t27975 + F::cast_from(0.61836467013888888889e-4_f64) * t27981;
    (t29011, t29022)
}
