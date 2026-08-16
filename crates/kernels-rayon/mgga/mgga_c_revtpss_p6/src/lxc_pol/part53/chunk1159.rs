//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1159/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1159(t1381: f64, t8590: f64, t98040: f64, t121031: f64, t121242: f64, t121249: f64, t121251: f64, t121259: f64, t121273: f64, t121276: f64, t121287: f64, t121308: f64, t121313: f64, t121327: f64, t121334: f64, t125861: f64, t125868: f64, t125873: f64, t125875: f64, t2022: f64, t27853: f64, t27960: f64, t32233: f64, t32255: f64, t32700: f64, t33952: f64, t543: f64, t5658: f64, t8578: f64, t8706: f64, t8707: f64) -> f64 {
    let t125886 = t98040 * t8590 * t1381;
    let t125894 = -0.50779446784275991476e-1_f64 * t121242 + 0.18822977838986977999e-4_f64 * t121249 - 0.17347256376410398924e1_f64 * t32233 * t125861 + 0.17347256376410398924e1_f64 * t121031 * t27853 + 0.50779446784275991476e-1_f64 * t121251 - t121259 - t121273 + t121276 + 0.3718732920905101082e-3_f64 * t125868 + t121287 + 0.3718732920905101082e-3_f64 * t125873 + 0.7437465841810202164e-3_f64 * t125875 + t121308 - 0.11423947533020470523e1_f64 * t32700 * t33952 - 0.28912093960683998208e-1_f64 * t121313 - 0.11423947533020470523e1_f64 * t8706 * t32255 * t8578 * t5658 * t543 + 0.1859366460452550541e-4_f64 * t125886 + 0.11423947533020470523e1_f64 * t8706 * t8707 * t2022 * t27960 - 0.33467254597718846885e-4_f64 * t121327 + 0.28559868832551176308e-1_f64 * t121334;
    t125894
}
