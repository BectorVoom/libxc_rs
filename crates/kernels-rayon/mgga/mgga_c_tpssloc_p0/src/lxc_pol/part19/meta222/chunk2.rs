//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 925/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk925(t10768: f64, t10847: f64, t300: f64, t2940: f64, t2944: f64, t2924: f64, t2929: f64, t4497: f64, t959: f64, t10665: f64, t10699: f64, t10707: f64, t10711: f64, t10715: f64, t10729: f64, t10733: f64, t10739: f64, t10819: f64) -> (f64, f64, f64, f64, f64) {
    let t10849 = t300 * (t10768 + t10847);
    let t10851 = 0.35089341735807877242e1_f64 * t2940 * t2944;
    let t10853 = t2929 * t2924 * t4497;
    let t10855 = 0.51947577317044391277e2_f64 * t959 * t10853;
    let t10856 = -t10665 + t10699 + t10707 - t10711 + t10715 + t10849 - t10819 + t10739 - t10729 + t10733 + t10851 - t10855;
    (t10849, t10851, t10853, t10855, t10856)
}
