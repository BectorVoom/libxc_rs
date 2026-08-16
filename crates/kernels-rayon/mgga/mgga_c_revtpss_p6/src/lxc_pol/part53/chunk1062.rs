//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1062/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1062(t1903: f64, t32211: f64, t5673: f64, t32206: f64, t545: f64, t25876: f64, t1883: f64, t7301: f64, t32188: f64, t32191: f64, t32203: f64, t32222: f64, t32225: f64, t32226: f64, t32233: f64, t32242: f64, t33923: f64, t33927: f64, t7930: f64, t8579: f64) -> (f64, f64, f64) {
    let t33930 = t5673 * t32211 * t1903;
    let t33931 = t32206 * t33930;
    let t33935 = t545 * t1903;
    let t33936 = t25876 * t33935;
    let t33939 = t7301 * t1883;
    let t33942 = -t32188 + t32191 - 0.28234466758480466999e-3_f64 * t33923 - t32203 - 0.112937867033921868e-2_f64 * t33927 - 0.28234466758480466999e-3_f64 * t33931 + t32222 - t32225 - 0.17347256376410398924e1_f64 * t32226 * t7930 + 0.17347256376410398924e1_f64 * t8579 * t33936 + 0.8673628188205199462e0_f64 * t32233 * t33939 - t32242;
    (t33930, t33935, t33942)
}
