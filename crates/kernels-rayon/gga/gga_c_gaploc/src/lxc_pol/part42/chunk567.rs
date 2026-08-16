//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 567/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk567(t10024: f64, t10867: f64, t2714: f64, t3040: f64, t2718: f64, t9873: f64, t3500: f64, t7416: f64, t10827: f64, t2685: f64, t2684: f64, t2465: f64, t2958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10868 = t10867 * t10024;
    let t10869 = 0.44688112439813033337e-1_f64 * t10868;
    let t10871 = 0.35750489951850426669e0_f64 * t2714 * t3040;
    let t10873 = 0.35750489951850426669e0_f64 * t2718 * t3040;
    let t10876 = 0.15976219147466979032e-1_f64 * t9873;
    let t10877 = t7416 * t3500;
    let t10878 = 0.19171462976960374838e0_f64 * t10877;
    let t10879 = t2685 * t10827;
    let t10880 = t2684 * t10879;
    let t10881 = 0.19171462976960374838e0_f64 * t10880;
    let t10882 = t2465 * t2958;
    (t10868, t10869, t10871, t10873, t10876, t10877, t10878, t10880, t10881, t10882)
}
