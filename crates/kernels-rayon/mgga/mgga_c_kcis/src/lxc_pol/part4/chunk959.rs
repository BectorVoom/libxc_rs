//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 959/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk959(t3214: f64, t9429: f64, t3209: f64, t982: f64, t2865: f64, t359: f64, t169: f64, t2843: f64, t1131: f64, t3201: f64, t2861: f64, t3192: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9430 = t9429 * t3214;
    let t9438 = t3209 * t982;
    let t9476 = t2865 * t359;
    let t9494 = 1.0_f64 / t2843 / t169;
    let t9517 = t3201 * t1131;
    let t9522 = t2861 * t3192;
    (t9430, t9438, t9476, t9494, t9517, t9522)
}
