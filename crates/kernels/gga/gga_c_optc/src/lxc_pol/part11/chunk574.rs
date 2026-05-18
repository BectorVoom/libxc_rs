//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 574/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk574<F: Float>(t4625: F, t696: F, t4651: F, t4655: F, t2113: F, t2124: F, t2142: F, t2159: F, t2168: F, t2180: F, t3471: F, t3489: F, t3504: F, t3517: F, t4631: F, t4661: F, t4665: F, t4681: F, t4685: F, t4690: F, t4694: F, t4699: F, t4703: F, t673: F, t686: F, t695: F, t705: F) -> (F, F, F, F) {
    let t4706 = t696 * t4625;
    let t4712 = t696 * t4651;
    let t4715 = t696 * t4655;
    let t4723 = F::new(0.17386322979577515709e0) * t2113 * t4681 + F::new(0.40568086952347536654e0) * t3471 + F::new(0.34772645959155031418e0) * t2124 * t4685 - F::new(0.86931614897887578546e-1) * t673 * t4690 - F::new(0.86931614897887578546e-1) * t673 * t4694 + t2142 + F::new(0.81136173904695073308e0) * t3489 + F::new(0.52158968938732547127e0) * t686 * t4699 - F::new(0.17386322979577515709e0) * t686 * t4703 + F::new(0.45342634012527777558e-1) * t2159 * t4706 + F::new(0.1410659724834197524e0) * t3504 + F::new(0.12091369070007407349e0) * t2168 * t4631 - F::new(0.15114211337509259186e-1) * t695 * t4712 - F::new(0.15114211337509259186e-1) * t695 * t4715 + t2180 + F::new(0.2821319449668395048e0) * t3517 + F::new(0.15114211337509259186e0) * t705 * t4661 - F::new(0.30228422675018518372e-1) * t705 * t4665;
    (t4706, t4712, t4715, t4723)
}
