//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1170/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1170(t20773: f64, t34503: f64, t3712: f64, t34465: f64, t3714: f64, t11447: f64, t33490: f64, t11452: f64, t34484: f64, t34486: f64, t34489: f64, t34492: f64, t34495: f64, t34497: f64, t34499: f64, t34501: f64) -> (f64, f64) {
    let t34505 = t34503 * t3712 * t20773;
    let t34507 = t34465 * t3714;
    let t34509 = t11447 * t33490;
    let t34510 = t34509 * t11452;
    let t34512 = -0.53810508162887008105e-7_f64 * t34484 + 0.6629778687778673199e-7_f64 * t34486 - 0.2318836277704281739e-4_f64 * t34489 + 0.2318836277704281739e-4_f64 * t34492 + 0.78584976712469872988e-8_f64 * t34495 - 0.21103240995305505364e-7_f64 * t34497 - 0.70344136651018351214e-8_f64 * t34499 - 0.64087860648527174258e-6_f64 * t34501 + 0.2209926229259557733e-7_f64 * t34505 - 0.64087860648527174258e-6_f64 * t34507 - 0.98332751566569010432e-7_f64 * t34510;
    (t34509, t34512)
}
