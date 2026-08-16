//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1197/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1197(t1060: f64, t355: f64, t5506: f64, t721: f64, t1072: f64, t1734: f64, t3124: f64, t3126: f64, t16296: f64, t2297: f64, t4818: f64, t16288: f64, t16292: f64, t16294: f64, t16300: f64, t16304: f64, t21737: f64, t21740: f64, t21743: f64, t21745: f64, t21747: f64, t21751: f64) -> (f64, f64, f64, f64) {
    let t21755 = t1060 * t355 * t5506 * t721;
    let t21759 = t3124 * t1072 * t1734 * t3126;
    let t21762 = t16296 * t2297 * t4818;
    let t21769 = -0.7335e0_f64 * t21737 + 0.489e0_f64 * t21740 + 0.2445e0_f64 * t21743 - 0.489e0_f64 * t21745 + 0.2445e0_f64 * t21747 + 0.2445e0_f64 * t21751 - 0.12225e0_f64 * t21755 - 0.12225e0_f64 * t21759 - 0.8802e1_f64 * t21762 + 0.1956e1_f64 * t16288 - 0.489e0_f64 * t16292 - 0.21733333333333333333e1_f64 * t16294 + 0.978e0_f64 * t16300 - 0.12225e0_f64 * t16304;
    (t21755, t21759, t21762, t21769)
}
