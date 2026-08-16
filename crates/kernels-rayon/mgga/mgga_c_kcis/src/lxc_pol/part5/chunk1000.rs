//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1000/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1000(t359: f64, t9372: f64, t9494: f64, t3198: f64, t4992: f64, t86: f64, t5168: f64, t1018: f64, t1747: f64, t1017: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t13131 = t359 * t9372;
    let t13155 = t359 * t9494;
    let t13172 = t86 * t4992 * t3198;
    let t13181 = t5168 * sigma0;
    let t13190 = t1018 * t1747;
    let t13192 = t86 * t1017 * t13190;
    (t13131, t13155, t13172, t13181, t13192)
}
