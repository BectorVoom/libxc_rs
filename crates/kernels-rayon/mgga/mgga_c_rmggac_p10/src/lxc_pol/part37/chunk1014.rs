//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1014/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1014(t78367: f64, t75907: f64, t75910: f64, t70104: f64, t70106: f64, t70108: f64, t70110: f64, t75921: f64, t75936: f64, t75943: f64, t739: f64, t78112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78368 = 0.42564599893297839398e-5_f64 * t78367;
    let t78371 = 0.1276937996798935182e-4_f64 * t75907;
    let t78372 = 0.1276937996798935182e-4_f64 * t75910;
    let t78375 = 0.638468998399467591e-4_f64 * t70104;
    let t78376 = 0.1276937996798935182e-3_f64 * t70106;
    let t78377 = 0.1915406995198402773e-3_f64 * t70108;
    let t78378 = 0.638468998399467591e-4_f64 * t70110;
    let t78379 = 0.14967802127329760705e-1_f64 * t75921;
    let t78384 = 0.23268647941669485538e-4_f64 * t75936;
    let t78385 = 0.3192344991997337955e-4_f64 * t75943;
    let t78390 = t739 * t78112;
    (t78368, t78371, t78372, t78375, t78376, t78377, t78378, t78379, t78384, t78385, t78390)
}
