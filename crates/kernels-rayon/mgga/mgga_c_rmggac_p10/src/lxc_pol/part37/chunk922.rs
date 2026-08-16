//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 922/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk922(t76680: f64, t7720: f64, t73803: f64, t73805: f64, t73814: f64, t73819: f64, t73822: f64, t73837: f64, t73845: f64, t1971: f64, t3351: f64, t4617: f64, t9552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76681 = t7720 * t76680;
    let t76682 = 0.12769379967989351819e-4_f64 * t76681;
    let t76683 = 0.85129199786595678799e-5_f64 * t73803;
    let t76684 = 0.85129199786595678799e-5_f64 * t73805;
    let t76688 = 0.16351352353374609375e-5_f64 * t73814;
    let t76689 = 0.39726959900411316773e-4_f64 * t73819;
    let t76690 = 0.2553875993597870364e-4_f64 * t73822;
    let t76693 = 0.2553875993597870364e-4_f64 * t73837;
    let t76696 = 0.23268647941669485538e-4_f64 * t73845;
    let t76700 = t3351 * t1971 * t4617 * t9552;
    (t76682, t76683, t76684, t76688, t76689, t76690, t76693, t76696, t76700)
}
