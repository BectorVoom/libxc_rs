//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 995/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk995(t33643: f64, t944: f64, t157: f64, t556: f64, t929: f64, t5299: f64, t615: f64, t406: f64, t463: f64, t7884: f64, t8396: f64, t2137: f64, t32123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33644 = t33643 * t944;
    let t33651 = t556 * t929 * t157;
    let t33658 = t615 * t5299;
    let t33675 = t944 * t463 * t406;
    let t33682 = t7884 * t8396;
    let t33698 = t2137 * t32123;
    (t33644, t33651, t33658, t33675, t33682, t33698)
}
