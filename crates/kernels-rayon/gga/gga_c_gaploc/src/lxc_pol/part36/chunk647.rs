//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 647/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk647(t10024: f64, t10867: f64, t2714: f64, t3040: f64, t2718: f64, t10850: f64, t10853: f64, t10855: f64, t10859: f64, t10862: f64, t10864: f64, t10866: f64, t9812: f64, t9815: f64, t9822: f64, t9826: f64, t9832: f64) -> f64 {
    let t10868 = t10867 * t10024;
    let t10869 = 0.44688112439813033337e-1_f64 * t10868;
    let t10871 = 0.35750489951850426669e0_f64 * t2714 * t3040;
    let t10873 = 0.35750489951850426669e0_f64 * t2718 * t3040;
    let t10874 = -t10850 + t10853 - t10855 - t10859 - t10862 + t10864 + t10866 - t10869 + t10871 + t10873 + t9812 + t9815 - t9822 + t9826 + t9832;
    t10874
}
