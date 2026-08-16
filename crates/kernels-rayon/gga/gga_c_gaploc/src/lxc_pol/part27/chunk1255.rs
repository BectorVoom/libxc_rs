//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1255/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1255(t33222: f64, t24657: f64, t7372: f64, t2684: f64, t32803: f64, t7585: f64, t7427: f64, t7573: f64, t5679: f64, t7682: f64, t8792: f64, t2628: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33223 = 0.19171462976960374838e0_f64 * t33222;
    let t33224 = t24657 * t7372;
    let t33225 = 0.29792074959875355558e-1_f64 * t33224;
    let t33228 = 0.14953741122029092374e3_f64 * t2684 * t7585 * t32803;
    let t33231 = 0.37959496694381542179e3_f64 * t7427 * t7573 * t32803;
    let t33238 = 0.21450293971110256002e1_f64 * t5679 * t8792 * t7682;
    let t33239 = t8521 * t2628;
    (t33223, t33225, t33228, t33231, t33238, t33239)
}
