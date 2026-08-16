//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 721/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk721(t390: f64, t7746: f64, t7458: f64, t7487: f64, t1980: f64, t1967: f64, t2087: f64, t2092: f64, t1988: f64, t7476: f64, t7483: f64, t1973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7747 = t7746 * t390;
    let t7753 = t7458 * t7487;
    let t7754 = t1980 * t7753;
    let t7755 = 0.28582678745379824648e-3_f64 * t7754;
    let t7756 = t1967 * t2087;
    let t7758 = t1967 * t2092;
    let t7759 = 0.25724410870841842184e-2_f64 * t7758;
    let t7760 = t1988 * t2087;
    let t7761 = 0.10718504529517434243e-2_f64 * t7760;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7772 = 0.7145669686344956162e-3_f64 * t7771;
    let t7773 = t1967 * t1973;
    (t7747, t7753, t7755, t7756, t7759, t7761, t7770, t7772, t7773)
}
