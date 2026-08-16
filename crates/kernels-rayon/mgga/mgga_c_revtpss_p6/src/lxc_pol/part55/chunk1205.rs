//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1205/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1205(t126108: f64, t126112: f64, t121941: f64, t27186: f64, t121846: f64, t121851: f64, t126122: f64, t126134: f64, t126136: f64, t126141: f64, t126145: f64, t14495: f64, t27183: f64, t32434: f64, t32463: f64, t7398: f64, t7759: f64, t8649: f64, t8650: f64) -> f64 {
    let t127641 = 0.17354086964223805049e-2_f64 * t126108;
    let t127642 = 0.66119071333692697238e-4_f64 * t126112;
    let t127643 = t121941 * t27186;
    let t127659 = t127641 - t127642 - 0.14456046980341999104e-1_f64 * t127643 + 0.57119737665102352616e0_f64 * t8649 * t8650 * t7398 * t7759 - 0.11423947533020470523e1_f64 * t32463 * t121846 * t14495 + 0.7437465841810202164e-3_f64 * t126122 + t121851 + 0.17347256376410398924e1_f64 * t32434 * t27183 + 0.37645955677973955999e-4_f64 * t126134 - 0.66934509195437693771e-4_f64 * t126136 - 0.29749863367240808656e-2_f64 * t126141 - 0.7437465841810202164e-2_f64 * t126145;
    t127659
}
