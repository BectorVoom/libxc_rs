//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 574/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk574(t4625: f64, t696: f64, t4651: f64, t4655: f64, t2113: f64, t2124: f64, t2142: f64, t2159: f64, t2168: f64, t2180: f64, t3471: f64, t3489: f64, t3504: f64, t3517: f64, t4631: f64, t4661: f64, t4665: f64, t4681: f64, t4685: f64, t4690: f64, t4694: f64, t4699: f64, t4703: f64, t673: f64, t686: f64, t695: f64, t705: f64) -> (f64, f64, f64, f64) {
    let t4706 = t696 * t4625;
    let t4712 = t696 * t4651;
    let t4715 = t696 * t4655;
    let t4723 = 0.17386322979577515709e0_f64 * t2113 * t4681 + 0.40568086952347536654e0_f64 * t3471 + 0.34772645959155031418e0_f64 * t2124 * t4685 - 0.86931614897887578546e-1_f64 * t673 * t4690 - 0.86931614897887578546e-1_f64 * t673 * t4694 + t2142 + 0.81136173904695073308e0_f64 * t3489 + 0.52158968938732547127e0_f64 * t686 * t4699 - 0.17386322979577515709e0_f64 * t686 * t4703 + 0.45342634012527777558e-1_f64 * t2159 * t4706 + 0.1410659724834197524e0_f64 * t3504 + 0.12091369070007407349e0_f64 * t2168 * t4631 - 0.15114211337509259186e-1_f64 * t695 * t4712 - 0.15114211337509259186e-1_f64 * t695 * t4715 + t2180 + 0.2821319449668395048e0_f64 * t3517 + 0.15114211337509259186e0_f64 * t705 * t4661 - 0.30228422675018518372e-1_f64 * t705 * t4665;
    (t4706, t4712, t4715, t4723)
}
