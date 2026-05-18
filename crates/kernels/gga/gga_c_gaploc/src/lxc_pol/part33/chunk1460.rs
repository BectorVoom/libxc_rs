//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1460/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1460<F: Float>(t1383: F, t3718: F, t4349: F, t331: F, t33991: F, t33997: F, t34003: F, t34008: F, t34010: F, t34012: F, t34018: F, t34020: F, t34023: F, t35240: F, t35242: F, t38880: F, t38881: F, t38924: F, t38930: F, t38953: F, t38954: F, t38958: F, t38983: F, t38993: F, t38998: F, t39026: F, t39032: F, t39039: F, t39055: F, t39076: F, t39101: F, t39106: F, t39111: F, t39126: F, t39136: F, t39140: F, t39154: F, t39170: F, t39208: F, t39246: F, t39249: F, t39261: F, t39268: F, t39281: F, t39294: F, t39299: F, t39302: F, t39321: F, t39330: F, t39339: F, t39342: F, t39361: F, t39362: F, t39375: F, t39376: F, t39383: F, t39407: F, t39420: F, t39425: F, t39435: F, t39438: F, t39440: F, t39464: F, t39467: F, t39468: F, t39493: F, t39511: F, t748: F) -> (F, F) {
    let t39519 = F::new(6.0) * t4349 * t3718 * t1383;
    let t39520 = t33991 - t748 * (t39330 + t39321 + t39302 + t39299 + t39294 + t39281 + t39268 + t39261 + t39249 + t39246 + t39208 + t39170 + t39154 + t39140 + t39136 + t39126 + t39111 + t39106 + t39101 + t39076 + t39055 + t39039 + t39032 + t39026 + t38998 + t38993 + t38983 + t38958 + t38954 + t38953 + t38930 + t38924) - t33997 + t34003 - t34008 + t34010 - t34012 - t38880 + t38881 + t34018 - t34020 - t34023 - t39339 + t39342 + (t39361 + t39362 + t39375 + t39376 + t39383 + t39407 + t39420 + t39425 + t39435 + t39438 + t39440 + t39464 + t39467 + t39468 + t39493 + t39511) * t331 + t39519 + t35240 - t35242;
    (t39519, t39520)
}
