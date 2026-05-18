//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 439/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk439<F: Float>(t2182: F, t661: F, t2037: F, t2089: F, t2093: F, t2113: F, t2116: F, t2121: F, t2124: F, t2127: F, t2132: F, t2136: F, t2142: F, t2145: F, t2148: F, t2152: F, t2159: F, t2160: F, t2165: F, t2168: F, t2171: F, t2174: F, t2180: F, t673: F, t686: F, t695: F, t705: F) -> F {
    let t2183 = t2182 * t661;
    let t2189 = F::new(0.17386322979577515709e0) * t2113 * t2116 + F::new(0.40568086952347536654e0) * t2121 + F::new(0.34772645959155031418e0) * t2124 * t2127 - F::new(0.86931614897887578546e-1) * t673 * t2132 - F::new(0.86931614897887578546e-1) * t673 * t2136 + t2142 + F::new(0.81136173904695073308e0) * t2145 + F::new(0.52158968938732547127e0) * t686 * t2148 - F::new(0.17386322979577515709e0) * t686 * t2152 + F::new(0.45342634012527777558e-1) * t2159 * t2160 + F::new(0.1410659724834197524e0) * t2165 + F::new(0.12091369070007407349e0) * t2168 * t2037 - F::new(0.15114211337509259186e-1) * t695 * t2171 - F::new(0.15114211337509259186e-1) * t695 * t2174 + t2180 + F::new(0.2821319449668395048e0) * t2183 + F::new(0.15114211337509259186e0) * t705 * t2089 - F::new(0.30228422675018518372e-1) * t705 * t2093;
    t2189
}
