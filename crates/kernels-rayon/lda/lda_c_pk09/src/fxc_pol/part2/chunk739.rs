//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 739/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk739(t741: f64, t93: f64, t89: f64, t623: f64, t7647: f64, t121: f64, t2214: f64, t2222: f64, t2977: f64, t3138: f64, t3142: f64, t3265: f64, t709: f64, t713: f64, t7590: f64, t7619: f64, t7621: f64, t7625: f64, t7629: f64, t7634: f64, t7636: f64, t7640: f64, t7642: f64, t7650: f64) -> f64 {
    let t7652 = t93 * t741;
    let t7653 = t89 * t7652;
    let t7654 = t7647 * t623;
    let t7655 = t121 * t7654;
    let t7658 = 1.8805371096875316_f64 * t3265 * t7590 + 2.9824072957409817_f64 * t2222 * t3138 + 2.9824072957409817_f64 * t2222 * t3142 - 0.15277772349540736_f64 * t7619 * t7621 + 5.9648145914819635_f64 * t7625 * t7621 + 2.9824072957409817_f64 * t7629 - 4.937333717448355_f64 * t2977 * t2214 - 4.937333717448355_f64 * t7634 * t7636 - 4.937333717448355_f64 * t7640 - 4.937333717448355_f64 * t7642 * t713 - 4.937333717448355_f64 * t7642 * t709 - 0.04115066352984959_f64 * t7650 - 0.08230132705969918_f64 * t7653 * t7655;
    t7658
}
