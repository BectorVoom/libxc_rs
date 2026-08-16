//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 790/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk790(t5774: f64, t951: f64, t2912: f64, t2919: f64, t4335: f64, t4384: f64, t5679: f64, t5683: f64, t5687: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64, t5718: f64, t5721: f64, t5724: f64) -> (f64, f64) {
    let t5775 = t5774 * t951;
    let t5790 = -0.1294625e1_f64 * t5699 + 0.258925e1_f64 * t5706 + t2912 + 0.20128333333333333334e0_f64 * t4335 - 0.20128333333333333333e0_f64 * t5679 + 0.60385e0_f64 * t5683 - 0.301925e0_f64 * t5687 + 0.82524375e-1_f64 * t5712 + 0.16504875e0_f64 * t5714 + t2919 + 0.11038e0_f64 * t4384 - 0.27595e-1_f64 * t5718 + 0.16557e0_f64 * t5721 - 0.82785e-1_f64 * t5724;
    (t5775, t5790)
}
