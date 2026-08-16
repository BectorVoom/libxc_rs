//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1086/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1086(t10684: f64, t38355: f64, t10648: f64, t10958: f64, t10971: f64, t10962: f64, t37599: f64, t37822: f64, t37833: f64, t37919: f64, t38001: f64, t38054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38356 = t38355 * t10684;
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38452 = 0.42952285777298855708e-4_f64 * t37599;
    let t38528 = 0.14224135994914204065e1_f64 * t37822;
    let t38532 = 0.17888640988868435534e-2_f64 * t37833;
    let t38568 = 0.18496169001454677638e1_f64 * t37919;
    let t38597 = 0.31806003678208078381e-2_f64 * t38001;
    let t38617 = 0.39552774754617995815e1_f64 * t38054;
    (t38356, t38359, t38362, t38452, t38528, t38532, t38568, t38597, t38617)
}
