//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 974/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk974(t36662: f64, t8417: f64, t1986: f64, t305: f64, t495: f64, t552: f64, t7717: f64, t38471: f64, t7473: f64, t7478: f64, t35637: f64, t1971: f64, t236: f64, t5620: f64, t7365: f64) -> (f64, f64, f64, f64, f64) {
    let t40654 = t36662 * t8417;
    let t40655 = 0.39726959900411316772e-4_f64 * t40654;
    let t40658 = t1986 * t305 * t552 * t495;
    let t40659 = t7717 * t40658;
    let t40661 = t38471 * t7473;
    let t40662 = t40661 * t7478;
    let t40664 = t35637 * t8417;
    let t40668 = t7365 * t1971 * t236 * t5620;
    (t40655, t40659, t40662, t40664, t40668)
}
