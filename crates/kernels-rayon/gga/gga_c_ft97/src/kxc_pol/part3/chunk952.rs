//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 952/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk952(t4635: f64, t766: f64, t2607: f64, t2606: f64, t505: f64, t3885: f64, t3892: f64, t3891: f64, t10085: f64, t5166: f64, t3821: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18707 = t4635 * t766;
    let t18708 = t2607 * t18707;
    let t18709 = t2606 * t18708;
    let t18712 = t4635 * t505;
    let t18713 = t3885 * t18712;
    let t18714 = t2606 * t18713;
    let t18717 = t3892 * t18712;
    let t18718 = t3891 * t18717;
    let t18721 = t10085 * t5166;
    let t18724 = t992 * t3821;
    (t18709, t18712, t18714, t18718, t18721, t18724)
}
