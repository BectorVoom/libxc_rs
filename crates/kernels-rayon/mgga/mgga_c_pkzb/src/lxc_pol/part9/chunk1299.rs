//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1299/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1299(t2196: f64, t3030: f64, t2199: f64, t1171: f64, t6141: f64, t6144: f64, t2256: f64, t3080: f64, t1189: f64, t6312: f64, t2240: f64, t2242: f64, t8003: f64, t851: f64) -> (f64, f64, f64, f64, f64) {
    let t22820 = t3030 * t2196;
    let t22822 = 6.0_f64 * t22820 * t2199;
    let t22823 = t1171 * t6141;
    let t22825 = 0.96491876992155210402e2_f64 * t22823 * t6144;
    let t22826 = t3080 * t2256;
    let t22829 = t1189 * t6312;
    let t22837 = 0.48245938496077605201e2_f64 * t2240 * t8003 * t2242 * t851;
    (t22822, t22825, t22826, t22829, t22837)
}
