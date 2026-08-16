//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 737/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk737(t390: f64, t7746: f64, t1020: f64, t2001: f64, t1029: f64, t7458: f64, t7487: f64, t1980: f64, t1967: f64, t2087: f64, t2092: f64, t1988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7747 = t7746 * t390;
    let t7748 = 0.40015750243531754508e-2_f64 * t7747;
    let t7749 = t2001 * t1020;
    let t7751 = t2001 * t1029;
    let t7753 = t7458 * t7487;
    let t7754 = t1980 * t7753;
    let t7755 = 0.28582678745379824648e-3_f64 * t7754;
    let t7756 = t1967 * t2087;
    let t7758 = t1967 * t2092;
    let t7759 = 0.25724410870841842184e-2_f64 * t7758;
    let t7760 = t1988 * t2087;
    (t7748, t7749, t7751, t7753, t7754, t7755, t7756, t7758, t7759, t7760)
}
