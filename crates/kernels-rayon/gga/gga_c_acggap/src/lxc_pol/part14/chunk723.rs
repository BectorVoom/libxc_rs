//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 723/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk723(t390: f64, t7746: f64, t7458: f64, t7487: f64, t1980: f64, t1967: f64, t2087: f64, t2092: f64, t1988: f64, t7476: f64, t7483: f64, t1973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7747 = t7746 * t390;
    let t7748 = 0.40015750243531754508e-2_f64 * t7747;
    let t7753 = t7458 * t7487;
    let t7754 = t1980 * t7753;
    let t7756 = t1967 * t2087;
    let t7758 = t1967 * t2092;
    let t7760 = t1988 * t2087;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7773 = t1967 * t1973;
    (t7748, t7753, t7754, t7756, t7758, t7760, t7770, t7771, t7773)
}
