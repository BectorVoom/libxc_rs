//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1035/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1035(t27596: f64, t33384: f64, t1095: f64, t218: f64, t51: f64, t6783: f64, t27703: f64, t123124: f64, t33365: f64, t35466: f64, t666: f64, t7477: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150764 = t33384 * t27596;
    let t150770 = t6783 * t51 * t218 * t1095;
    let t150773 = t27703 * t6783;
    let t150776 = t123124 * t33365;
    let t150786 = t35466 * t666;
    let t150787 = t7477 * t150786;
    (t150764, t150770, t150773, t150776, t150786, t150787)
}
