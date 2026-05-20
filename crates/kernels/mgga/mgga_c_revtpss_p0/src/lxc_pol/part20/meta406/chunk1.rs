//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1502/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502<F: Float>(t1011: F, t1012: F, t1015: F, t1063: F, t1066: F, t11829: F, t11853: F, t11913: F, t247: F, t3188: F, t3241: F, t39443: F, t39457: F, t41271: F, t41318: F, t42496: F, t42499: F, t42506: F, t42508: F, t42516: F, t42518: F) -> F {
    let t42529 = -F::cast_from(0.76220476654346199062e-2_f64) * t1063 * t247 * t11853 * t41271 + F::cast_from(0.38110238327173099531e-3_f64) * t42496 + t42499 / F::new(216.0) + t1011 * t1012 * t1015 * t39457 / F::new(288.0) + F::new(7.0) / F::new(486.0) * t42506 - F::new(7.0) / F::new(54.0) * t1011 * t1012 * t42508 * t39443 + F::new(8.0) / F::new(27.0) * t3241 * t11829 - t42516 / F::new(27.0) + t1011 * t1012 * t42518 * t39443 / F::new(6.0) - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t247 * t1066 * t41318 - F::cast_from(0.57165357490759649296e-2_f64) * t3188 * t11913;
    t42529
}
