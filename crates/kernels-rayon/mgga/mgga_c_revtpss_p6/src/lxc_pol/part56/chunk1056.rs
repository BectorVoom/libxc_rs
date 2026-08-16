//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1056/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1056(t121140: f64, t121142: f64, t25898: f64, t7063: f64, t8578: f64, t4104: f64, t550: f64, t561: f64, t9794: f64, t2453: f64, t8571: f64, t240: f64, t27: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121144 = 0.50779446784275991476e-2_f64 * t121140 * t121142;
    let t121146 = t7063 * t8578 * t25898;
    let t121147 = t121146 * t4104;
    let t121165 = t550 * t561;
    let t121166 = t9794 * t121165;
    let t121167 = t2453 * t8571 * t121166;
    let t121168 = 0.13386901839087538753e-4_f64 * t121167;
    let t121173 = t545 * t27 * t240;
    (t121144, t121146, t121147, t121165, t121166, t121168, t121173)
}
