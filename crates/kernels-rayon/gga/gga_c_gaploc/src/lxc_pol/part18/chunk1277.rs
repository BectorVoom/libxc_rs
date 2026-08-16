//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1277/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1277(t24321: f64, t787: f64, t9824: f64, t10677: f64, t10831: f64, t10954: f64, t1445: f64, t1865: f64, t28281: f64, t28284: f64, t32984: f64, t32987: f64, t32991: f64, t32997: f64, t33001: f64, t33004: f64, t33009: f64, t33013: f64, t33018: f64, t33021: f64, t4614: f64, t5676: f64, t813: f64) -> f64 {
    let t33023 = t787 * t24321 * t9824;
    let t33024 = 0.14896037479937677779e-1_f64 * t33023;
    let t33025 = -t32984 - t32987 + t32991 - 0.92023022289409799224e1_f64 * t813 * t1445 * t10677 * t1865 + t32997 - t33001 + t33004 - 0.12269736305254639896e2_f64 * t813 * t4614 * t10954 + t28281 - t33009 - t33013 + 0.79445533226334281486e-1_f64 * t5676 * t10831 + t33018 + t33021 + t33024 - t28284;
    t33025
}
