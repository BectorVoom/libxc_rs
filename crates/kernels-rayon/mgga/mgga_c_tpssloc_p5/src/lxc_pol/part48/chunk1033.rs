//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1033/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1033(t117407: f64, t117410: f64, t117412: f64, t117416: f64, t117418: f64, t117420: f64, t117422: f64, t117430: f64, t117662: f64, t117671: f64, t117690: f64, t1396: f64, t1398: f64, t1404: f64, t2099: f64, t2105: f64, t2170: f64, t2174: f64, t24448: f64, t24486: f64, t24955: f64, t24977: f64, t3: f64, t32393: f64, t32415: f64, t3932: f64, t3946: f64, t580: f64, t7223: f64, t7240: f64, t7416: f64, t7426: f64, t8844: f64, t8852: f64) -> f64 {
    let tv4rho2sigma24 = 2.0_f64 * t117407 + t3932 * t8852 + 2.0_f64 * t117410 + 2.0_f64 * t117412 + 2.0_f64 * t7416 * t7240 + 2.0_f64 * t117416 + 2.0_f64 * t117418 + 2.0_f64 * t117420 + 2.0_f64 * t117422 + t2170 * t24486 + 2.0_f64 * t32393 * t1404 + 2.0_f64 * t7223 * t7426 + t8844 * t3946 + 2.0_f64 * t117430 + t24955 * t2105 + t24448 * t2174 + t2099 * t24977 + 2.0_f64 * t1396 * t32415 + t3 * t117662 * t580 + t1398 * (t117671 + t117690);
    tv4rho2sigma24
}
