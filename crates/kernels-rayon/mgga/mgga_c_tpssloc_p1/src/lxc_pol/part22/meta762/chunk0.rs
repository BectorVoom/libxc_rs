//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2564/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564(t1117: f64, t11190: f64, t21724: f64, t3313: f64, t4781: f64, t5989: f64, t11424: f64, t21895: f64, t1147: f64, t21826: f64, t1128: f64, t21975: f64) -> (f64, f64, f64, f64, f64) {
    let t71850 = 24.0_f64 * t11190 * t21724 * t1117;
    let t71853 = 18.0_f64 * t3313 * t5989 * t4781;
    let t71855 = 6.0_f64 * t11424 * t21895;
    let t71860 = t21826 * t1147;
    let t71863 = t21975 * t1128;
    (t71850, t71853, t71855, t71860, t71863)
}
