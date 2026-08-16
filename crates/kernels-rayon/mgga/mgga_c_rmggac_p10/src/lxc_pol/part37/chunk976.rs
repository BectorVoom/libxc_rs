//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 976/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk976(t75564: f64, t75566: f64, t75575: f64, t75580: f64, t75583: f64, t75585: f64, t75587: f64, t14589: f64, t8533: f64, t75598: f64, t14424: f64, t4985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77659 = 0.2627895913935205078e-5_f64 * t75564;
    let t77660 = 0.2627895913935205078e-5_f64 * t75566;
    let t77664 = 0.10248087766267884741e-3_f64 * t75575;
    let t77665 = 0.38430329123504567781e-4_f64 * t75580;
    let t77666 = 0.72042316457491791901e-3_f64 * t75583;
    let t77669 = 0.1276937996798935182e-3_f64 * t75585;
    let t77670 = 0.1915406995198402773e-3_f64 * t75587;
    let t77671 = t14589 * t8533;
    let t77672 = 0.18183107769496894486e-1_f64 * t77671;
    let t77677 = 0.15961724959986689775e-4_f64 * t75598;
    let t77679 = 0.11974241701863808564e0_f64 * t4985 * t14424;
    (t77659, t77660, t77664, t77665, t77666, t77669, t77670, t77672, t77677, t77679)
}
