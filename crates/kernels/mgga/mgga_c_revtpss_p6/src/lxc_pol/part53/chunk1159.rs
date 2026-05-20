//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1159/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1159<F: Float>(t1381: F, t8590: F, t98040: F, t121031: F, t121242: F, t121249: F, t121251: F, t121259: F, t121273: F, t121276: F, t121287: F, t121308: F, t121313: F, t121327: F, t121334: F, t125861: F, t125868: F, t125873: F, t125875: F, t2022: F, t27853: F, t27960: F, t32233: F, t32255: F, t32700: F, t33952: F, t543: F, t5658: F, t8578: F, t8706: F, t8707: F) -> F {
    let t125886 = t98040 * t8590 * t1381;
    let t125894 = -F::cast_from(0.50779446784275991476e-1_f64) * t121242 + F::cast_from(0.18822977838986977999e-4_f64) * t121249 - F::cast_from(0.17347256376410398924e1_f64) * t32233 * t125861 + F::cast_from(0.17347256376410398924e1_f64) * t121031 * t27853 + F::cast_from(0.50779446784275991476e-1_f64) * t121251 - t121259 - t121273 + t121276 + F::cast_from(0.3718732920905101082e-3_f64) * t125868 + t121287 + F::cast_from(0.3718732920905101082e-3_f64) * t125873 + F::cast_from(0.7437465841810202164e-3_f64) * t125875 + t121308 - F::cast_from(0.11423947533020470523e1_f64) * t32700 * t33952 - F::cast_from(0.28912093960683998208e-1_f64) * t121313 - F::cast_from(0.11423947533020470523e1_f64) * t8706 * t32255 * t8578 * t5658 * t543 + F::cast_from(0.1859366460452550541e-4_f64) * t125886 + F::cast_from(0.11423947533020470523e1_f64) * t8706 * t8707 * t2022 * t27960 - F::cast_from(0.33467254597718846885e-4_f64) * t121327 + F::cast_from(0.28559868832551176308e-1_f64) * t121334;
    t125894
}
