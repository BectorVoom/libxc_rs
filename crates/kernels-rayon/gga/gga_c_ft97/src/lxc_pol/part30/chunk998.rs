//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 998/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk998(t27775: f64, t33460: f64, t24432: f64, t24437: f64, t1091: f64, t140714: f64, t2354: f64, t6118: f64, t24543: f64, t35350: f64, t2: f64, t35516: f64) -> (f64, f64, f64, f64, f64) {
    let t150071 = t33460 * t27775;
    let t150073 = t24437 * t24432 * t150071;
    let t150077 = t6118 * t2354 * t140714 * t1091;
    let t150079 = t24543 * t35350;
    let t150081 = t2 * t35516;
    (t150071, t150073, t150077, t150079, t150081)
}
