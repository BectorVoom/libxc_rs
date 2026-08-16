//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 985/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk985(t75736: f64, t75739: f64, t1550: f64, t2228: f64, t2347: f64, t69130: f64, t2211: f64, t8794: f64, t118: f64, t15547: f64, t2085: f64, t76143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77824 = 0.10909864661698136691e0_f64 * t75736;
    let t77825 = 0.21819729323396273382e0_f64 * t75739;
    let t77827 = t1550 * t2228 * t2347;
    let t77828 = 0.2993560425465952141e-1_f64 * t77827;
    let t77830 = 0.18183107769496894487e-1_f64 * t69130;
    let t77831 = t2211 * t8794;
    let t77833 = 0.39914139006212695214e-1_f64 * t118 * t77831;
    let t77834 = t15547 * t2085;
    let t77835 = 0.90915538847484472429e-2_f64 * t77834;
    let t77836 = 0.44903406381989282115e-1_f64 * t76143;
    (t77824, t77825, t77828, t77830, t77831, t77833, t77835, t77836)
}
