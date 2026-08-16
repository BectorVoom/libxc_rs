//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1003/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1003(t9873: f64, t3500: f64, t7416: f64, t10827: f64, t2685: f64, t2684: f64, t2465: f64, t2958: f64, t2464: f64, t787: f64, t8788: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10876 = 0.15976219147466979032e-1_f64 * t9873;
    let t10877 = t7416 * t3500;
    let t10878 = 0.19171462976960374838e0_f64 * t10877;
    let t10879 = t2685 * t10827;
    let t10880 = t2684 * t10879;
    let t10881 = 0.19171462976960374838e0_f64 * t10880;
    let t10882 = t2465 * t2958;
    let t10883 = t2464 * t10882;
    let t10884 = t2684 * t10883;
    let t10885 = 0.42603251059911944084e-1_f64 * t10884;
    let t10886 = t787 * t8788;
    let t10887 = t10886 * t9824;
    (t10876, t10878, t10879, t10881, t10882, t10883, t10885, t10886, t10887)
}
