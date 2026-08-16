//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 439/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk439(t2182: f64, t661: f64, t2037: f64, t2089: f64, t2093: f64, t2113: f64, t2116: f64, t2121: f64, t2124: f64, t2127: f64, t2132: f64, t2136: f64, t2142: f64, t2145: f64, t2148: f64, t2152: f64, t2159: f64, t2160: f64, t2165: f64, t2168: f64, t2171: f64, t2174: f64, t2180: f64, t673: f64, t686: f64, t695: f64, t705: f64) -> f64 {
    let t2183 = t2182 * t661;
    let t2189 = 0.17386322979577515709e0_f64 * t2113 * t2116 + 0.40568086952347536654e0_f64 * t2121 + 0.34772645959155031418e0_f64 * t2124 * t2127 - 0.86931614897887578546e-1_f64 * t673 * t2132 - 0.86931614897887578546e-1_f64 * t673 * t2136 + t2142 + 0.81136173904695073308e0_f64 * t2145 + 0.52158968938732547127e0_f64 * t686 * t2148 - 0.17386322979577515709e0_f64 * t686 * t2152 + 0.45342634012527777558e-1_f64 * t2159 * t2160 + 0.1410659724834197524e0_f64 * t2165 + 0.12091369070007407349e0_f64 * t2168 * t2037 - 0.15114211337509259186e-1_f64 * t695 * t2171 - 0.15114211337509259186e-1_f64 * t695 * t2174 + t2180 + 0.2821319449668395048e0_f64 * t2183 + 0.15114211337509259186e0_f64 * t705 * t2089 - 0.30228422675018518372e-1_f64 * t705 * t2093;
    t2189
}
