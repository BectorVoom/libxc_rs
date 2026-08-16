//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 480/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk480(t13862: f64, t354: f64, t3133: f64, t1993: f64, t3140: f64, t1995: f64, t305: f64, t1986: f64, t2002: f64, t2001: f64, t3141: f64, t322: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13863 = t13862 * t354;
    let t13864 = t3133 * t13863;
    let t13866 = t1993 * t3140;
    let t13867 = t305 * t1995;
    let t13868 = t1986 * t13867;
    let t13869 = t13866 * t13868;
    let t13871 = t305 * t2002;
    let t13872 = t2001 * t13871;
    let t13873 = t3141 * t13872;
    let t13875 = t793 * t322;
    (t13863, t13864, t13866, t13868, t13869, t13872, t13873, t13875)
}
