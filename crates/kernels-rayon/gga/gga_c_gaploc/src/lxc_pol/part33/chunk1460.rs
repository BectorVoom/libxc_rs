//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1460/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1460(t1383: f64, t3718: f64, t4349: f64, t331: f64, t33991: f64, t33997: f64, t34003: f64, t34008: f64, t34010: f64, t34012: f64, t34018: f64, t34020: f64, t34023: f64, t35240: f64, t35242: f64, t38880: f64, t38881: f64, t38924: f64, t38930: f64, t38953: f64, t38954: f64, t38958: f64, t38983: f64, t38993: f64, t38998: f64, t39026: f64, t39032: f64, t39039: f64, t39055: f64, t39076: f64, t39101: f64, t39106: f64, t39111: f64, t39126: f64, t39136: f64, t39140: f64, t39154: f64, t39170: f64, t39208: f64, t39246: f64, t39249: f64, t39261: f64, t39268: f64, t39281: f64, t39294: f64, t39299: f64, t39302: f64, t39321: f64, t39330: f64, t39339: f64, t39342: f64, t39361: f64, t39362: f64, t39375: f64, t39376: f64, t39383: f64, t39407: f64, t39420: f64, t39425: f64, t39435: f64, t39438: f64, t39440: f64, t39464: f64, t39467: f64, t39468: f64, t39493: f64, t39511: f64, t748: f64) -> (f64, f64) {
    let t39519 = 6.0_f64 * t4349 * t3718 * t1383;
    let t39520 = t33991 - t748 * (t39330 + t39321 + t39302 + t39299 + t39294 + t39281 + t39268 + t39261 + t39249 + t39246 + t39208 + t39170 + t39154 + t39140 + t39136 + t39126 + t39111 + t39106 + t39101 + t39076 + t39055 + t39039 + t39032 + t39026 + t38998 + t38993 + t38983 + t38958 + t38954 + t38953 + t38930 + t38924) - t33997 + t34003 - t34008 + t34010 - t34012 - t38880 + t38881 + t34018 - t34020 - t34023 - t39339 + t39342 + (t39361 + t39362 + t39375 + t39376 + t39383 + t39407 + t39420 + t39425 + t39435 + t39438 + t39440 + t39464 + t39467 + t39468 + t39493 + t39511) * t331 + t39519 + t35240 - t35242;
    (t39519, t39520)
}
