//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1097/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1097(t35006: f64, t92: f64, t138411: f64, t138445: f64, t138706: f64, t139563: f64, t1969: f64, t23413: f64, t24080: f64, t26801: f64, t26815: f64, t26817: f64, t26822: f64, t26950: f64, t27416: f64, t32714: f64, t32717: f64, t32724: f64, t3450: f64, t34975: f64, t40830: f64, t5772: f64, t5773: f64, t5775: f64, t6584: f64, t925: f64, t9432: f64) -> f64 {
    let t147073 = t35006 * t92;
    let t147091 = -t5772 * t138445 * t27416 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t5772 * t24080 * t26822 - t138411 / 27.0_f64 - t5772 * t1969 * t139563 * t925 / 9.0_f64 - t26817 * t32724 / 18.0_f64 - t147073 * t5775 / 18.0_f64 + 2.0_f64 * t5772 * t9432 * t5773 * t26950 + t32714 * t26801 / 9.0_f64 - t138706 * t6584 / 18.0_f64 - 4.0_f64 * t5772 * t40830 * t32717 * t3450 + t32714 * t26815 - t23413 * t34975 / 18.0_f64;
    t147091
}
