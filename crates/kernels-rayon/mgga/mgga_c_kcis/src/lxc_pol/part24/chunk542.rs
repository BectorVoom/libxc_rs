//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 542/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk542(t1131: f64, t4984: f64, t1096: f64, t1092: f64, t1713: f64, t2825: f64, t1020: f64, t251: f64, t66: f64, t1018: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4985 = t1131 * t4984;
    let t4986 = t1096 * t4985;
    let t4987 = t1092 * t4986;
    let t4989 = t2825 * t1713;
    let t4990 = t1020 * t4989;
    let t4992 = t66 * t251;
    let t4994 = t86 * t4992 * t1018;
    (t4985, t4986, t4987, t4989, t4990, t4992, t4994)
}
