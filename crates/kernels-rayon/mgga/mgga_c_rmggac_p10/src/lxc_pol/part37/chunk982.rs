//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 982/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk982(t7717: f64, t77768: f64, t75675: f64, t75681: f64, t75685: f64, t75687: f64, t75705: f64, t1356: f64, t37423: f64, t8936: f64, t14451: f64, t5267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77769 = t7717 * t77768;
    let t77770 = 0.53205749866622299248e-5_f64 * t77769;
    let t77772 = 0.79828278012425390427e-1_f64 * t75675;
    let t77773 = 0.1276937996798935182e-4_f64 * t75681;
    let t77774 = 0.15961724959986689775e-4_f64 * t75685;
    let t77775 = 0.1276937996798935182e-4_f64 * t75687;
    let t77782 = 0.44903406381989282115e-1_f64 * t75705;
    let t77785 = 0.11974241701863808564e0_f64 * t1356 * t37423 * t8936;
    let t77786 = t14451 * t5267;
    (t77770, t77772, t77773, t77774, t77775, t77782, t77785, t77786)
}
