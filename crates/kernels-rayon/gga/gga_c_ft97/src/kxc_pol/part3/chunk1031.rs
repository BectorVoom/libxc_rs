//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1031/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1031(t299: f64, t19904: f64, t19939: f64, t10947: f64, t10948: f64, t10949: f64, t10950: f64, t13: f64, t16584: f64, t17681: f64, t18793: f64, t4640: f64, t4905: f64, t5197: f64, t5490: f64) -> f64 {
    let t300 = 10000000.0_f64 <= t299;
    let t19941 = piecewise3(t300, 0.0_f64, t19904 + t19939);
    let tv3rho32 = t10947 + t10948 + t10949 + t10950 + t4640 + t4905 + t5197 + t5490 + t13 * (t16584 + t17681 + t18793 + t19941);
    tv3rho32
}
