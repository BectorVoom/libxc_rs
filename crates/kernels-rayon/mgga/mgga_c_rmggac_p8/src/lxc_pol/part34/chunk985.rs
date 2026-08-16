//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 985/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk985(t71210: f64, t74961: f64, t74965: f64, t14623: f64, t6355: f64, t14626: f64, t5055: f64, t2039: f64, t2479: f64, t270: f64, t638: f64, t2046: f64, t2050: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77299 = 0.36021158228745895953e-3_f64 * t71210;
    let t77300 = 0.20455996240684006298e-1_f64 * t74961;
    let t77301 = 0.2727466165424534173e-1_f64 * t74965;
    let t77302 = t6355 * t14623;
    let t77303 = 0.2993560425465952141e-1_f64 * t77302;
    let t77304 = t5055 * t14626;
    let t77305 = 0.44903406381989282115e-1_f64 * t77304;
    let t77308 = t638 * t2039 * t2479 * t270;
    let t77309 = 0.15243824895787514157e-3_f64 * t77308;
    let t77312 = t2046 * t2050 * t2479 * t31;
    (t77299, t77300, t77301, t77303, t77305, t77309, t77312)
}
