//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 636/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk636(t1616: f64, t3859: f64, t3666: f64, t3671: f64, t3676: f64, t3681: f64, t3685: f64, t3689: f64, t3692: f64, t3704: f64, t3710: f64, t3715: f64, t3719: f64) -> (f64, f64) {
    let t3861 = 2.0_f64 * t1616 * t3859;
    let t3873 = 0.80966145833333333339e-4_f64 * t3666 - 0.69504740211613770836e-3_f64 * t3671 - 0.50603841145833333338e-5_f64 * t3676 + 0.43440462632258606772e-4_f64 * t3681 - 0.4637672555408563478e-4_f64 * t3685 - 0.13506074236995523433e-5_f64 * t3689 + 0.11594181388521408695e-4_f64 * t3692 - 0.98332751566569010433e-8_f64 * t3704 + 0.42206481990611010728e-7_f64 * t3710 + 0.13259557375557346398e-6_f64 * t3715 - 0.18115908419564701085e-6_f64 * t3719;
    (t3861, t3873)
}
