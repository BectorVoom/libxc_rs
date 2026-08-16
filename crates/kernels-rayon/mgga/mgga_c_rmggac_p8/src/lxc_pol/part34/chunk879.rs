//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 879/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk879(t13989: f64, t39705: f64, t13993: f64, t39570: f64, t14004: f64, t44788: f64, t14170: f64, t26857: f64, t14368: f64, t15350: f64, t15411: f64, t68891: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75719 = t39705 * t13989;
    let t75721 = t39570 * t13993;
    let t75723 = t44788 * t14004;
    let t75725 = t26857 * t14170;
    let t75729 = t14368 * t15350;
    let t75733 = t68891 * t15411;
    (t75719, t75721, t75723, t75725, t75729, t75733)
}
