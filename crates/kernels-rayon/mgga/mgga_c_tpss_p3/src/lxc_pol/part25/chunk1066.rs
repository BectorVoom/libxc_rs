//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1066/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1066(t14632: f64, t895: f64, t904: f64, t912: f64, t2629: f64, t4961: f64, t11399: f64, t3907: f64, t10980: f64, t11002: f64, t11134: f64, t11135: f64, t14459: f64, t14492: f64, t14495: f64, t14505: f64, t14507: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t8616: f64, t8723: f64) -> (f64, f64, f64, f64) {
    let t14634 = t895 * t14632 * t904;
    let t14636 = 0.5848223622634646207e0_f64 * t912 * t14634;
    let t14638 = 0.17315859105681463759e2_f64 * t2629 * t4961;
    let t14639 = t3907 * t11399;
    let t14641 = 0.34631718211362927518e2_f64 * t912 * t14639;
    let t14656 = -t8723 - 0.79148148148148148147e-2_f64 * t8616 - 0.15829629629629629629e-1_f64 * t10980 + 0.79148148148148148147e-2_f64 * t11002 - t11134 + t11135 + 0.39574074074074074073e-2_f64 * t14495 - 0.19787037037037037037e-1_f64 * t14517 + 0.71233333333333333332e-1_f64 * t14459 - 0.23744444444444444444e-1_f64 * t14521 - 0.11872222222222222222e-1_f64 * t14505 - 0.10685e0_f64 * t14525 + 0.71233333333333333332e-1_f64 * t14528 + 0.5936111111111111111e-2_f64 * t14507 - 0.11872222222222222222e-1_f64 * t14532 + 0.35616666666666666666e-1_f64 * t14535 - 0.17808333333333333333e-1_f64 * t14492;
    (t14636, t14638, t14641, t14656)
}
