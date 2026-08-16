//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 873/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk873(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t4299: f64, t4302: f64, t4303: f64, t4304: f64, t7851: f64, t7855: f64) -> f64 {
    let t9097 = 2.2984542076810275_f64 * t7851 + 2.2984542076810275_f64 * t7855 - 0.15282509383508946_f64 * t3335 - 0.10188339589005964_f64 * t3342 + 4.596908415362055_f64 * t3384 + 4.596908415362055_f64 * t3388 - 4.596908415362055_f64 * t3393 + t4299 + t4302 + t4303 - t4304 + 0.15282509383508946_f64 * t3317 + 0.15282509383508946_f64 * t3319;
    t9097
}
