//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1478/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1478(t2924: f64, t952: f64, t2932: f64, t950: f64, t2836: f64, t914: f64, t2792: f64, t2844: f64, t912: f64, t2842: f64, t2880: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10720 = t952 * t2924;
    let t10723 = t2924 * t2932;
    let t10724 = t10723 * t950;
    let t10727 = t914 * t2836;
    let t10729 = 6.0_f64 * t2792 * t10727;
    let t10731 = t2836 * t2844 * t912;
    let t10733 = 0.48245938496077605201e2_f64 * t2842 * t10731;
    let t10734 = t933 * t2880;
    (t10720, t10723, t10724, t10727, t10729, t10731, t10733, t10734)
}
