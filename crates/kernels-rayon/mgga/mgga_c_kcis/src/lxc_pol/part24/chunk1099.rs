//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1099/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1099(t29010: f64, t4939: f64, t2173: f64, t26685: f64, t26856: f64, t27832: f64, t27967: f64, t27969: f64, t27975: f64, t27981: f64, t28988: f64, t28997: f64, t29001: f64, t29004: f64, t29007: f64, t7703: f64, t8038: f64) -> (f64, f64) {
    let t29011 = t4939 * t29010;
    let t29022 = t26856 - 0.13901041666666666667e-2_f64 * t2173 * t28997 - 0.55273148148148148147e-3_f64 * t29001 + 0.49745833333333333332e-2_f64 * t29004 - 0.23168402777777777778e-3_f64 * t7703 * t29007 - 0.30891203703703703704e-3_f64 * t7703 * t29011 - 0.18550940104166666667e-3_f64 * t26685 * t28988 - 0.46336805555555555556e-3_f64 * t27832 * t8038 - 0.46336805555555555556e-3_f64 * t27967 - 0.33163888888888888888e-2_f64 * t27969 + 0.46336805555555555556e-3_f64 * t27975 + 0.61836467013888888889e-4_f64 * t27981;
    (t29011, t29022)
}
