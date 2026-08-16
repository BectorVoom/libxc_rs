//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1265/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1265(t81742: f64, t6612: f64, t812: f64, t836: f64, t2690: f64, t6619: f64, t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64, t22822: f64, t281: f64, t6589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81743 = 0.43737152435318756759e-3_f64 * t81742;
    let t81749 = t812 * t6612 * t836;
    let t81763 = t812 * t6619 * t2690;
    let t81782 = t23121 * t131 * t9537;
    let t81783 = t81613 * t236;
    let t81788 = t22822 * t6589 * t281;
    (t81743, t81749, t81763, t81782, t81783, t81788)
}
