//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 948/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk948(t4263: f64, t466: f64, t161: f64, t8801: f64, t148: f64, t163: f64, t164: f64, t4130: f64, t1155: f64, t479: f64, t1062: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10750 = t466 * t4263;
    let t10752 = t8801 * t161;
    let t10755 = 0.031505407223141116_f64 * t148 * t10752 * t163;
    let t10757 = t4130 * t164;
    let t10760 = 0.7561297733553868_f64 * t1155 * t479;
    let t10762 = 1.0_f64 / t8 / t1062;
    (t10750, t10752, t10755, t10757, t10760, t10762)
}
