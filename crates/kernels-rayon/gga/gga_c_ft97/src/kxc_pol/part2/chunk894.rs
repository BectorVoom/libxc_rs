//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 894/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk894(t10079: f64, t13853: f64, t2567: f64, t668: f64, t2569: f64, t992: f64, t2606: f64, t2349: f64) -> (f64, f64, f64) {
    let t13854 = t10079 * t13853;
    let t13857 = t2567 * t668;
    let t13858 = t992 * t2569;
    let t13859 = t13857 * t13858;
    let t13860 = t2606 * t13859;
    let t13863 = t992 * t2349;
    (t13854, t13860, t13863)
}
