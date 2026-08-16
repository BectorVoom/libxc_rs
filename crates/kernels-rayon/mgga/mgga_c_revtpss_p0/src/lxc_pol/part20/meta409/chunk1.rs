//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1514/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514(t3115: f64, t3119: f64, t42793: f64, t11688: f64, t11922: f64, t4892: f64, t11249: f64, t3151: f64, t11722: f64, t3188: f64, t1011: f64, t11268: f64, t11639: f64, t11656: f64, t11678: f64, t11871: f64, t11927: f64, t11933: f64, t12017: f64, t16020: f64, t16025: f64, t16067: f64, t3117: f64, t3136: f64, t41314: f64, t42788: f64, t4915: f64) -> (f64, f64) {
    let t42795 = t3115 * t42793 * t3119;
    let t42798 = t4892 * t11922 * t11688;
    let t42804 = t3151 * t11249;
    let t42816 = t3188 * t11722;
    let t42820 = 0.1219527626469539185e-1_f64 * t42788 - 0.18292914397043087775e-1_f64 * t11656 * t11639 + 0.57165357490759649296e-3_f64 * t42795 + 0.34299214494455789578e-2_f64 * t42798 + 0.25724410870841842184e-2_f64 * t11927 * t3117 * t11678 * t16025 + 0.12862205435420921092e-2_f64 * t16067 * t3117 * t42804 * t16020 + 0.13719685797782315831e-1_f64 * t11933 * t12017 + 0.13719685797782315831e-1_f64 * t11933 * t11871 + t1011 * t4915 * t41314 / 8.0_f64 + 0.11433071498151929859e-2_f64 * t42816 + 0.43445671692977333464e-1_f64 * t11268 * t3136;
    (t42804, t42820)
}
