//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1514/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1514<F: Float>(t3115: F, t3119: F, t42793: F, t11688: F, t11922: F, t4892: F, t11249: F, t3151: F, t11722: F, t3188: F, t1011: F, t11268: F, t11639: F, t11656: F, t11678: F, t11871: F, t11927: F, t11933: F, t12017: F, t16020: F, t16025: F, t16067: F, t3117: F, t3136: F, t41314: F, t42788: F, t4915: F) -> (F, F) {
    let t42795 = t3115 * t42793 * t3119;
    let t42798 = t4892 * t11922 * t11688;
    let t42804 = t3151 * t11249;
    let t42816 = t3188 * t11722;
    let t42820 = F::cast_from(0.1219527626469539185e-1_f64) * t42788 - F::cast_from(0.18292914397043087775e-1_f64) * t11656 * t11639 + F::cast_from(0.57165357490759649296e-3_f64) * t42795 + F::cast_from(0.34299214494455789578e-2_f64) * t42798 + F::cast_from(0.25724410870841842184e-2_f64) * t11927 * t3117 * t11678 * t16025 + F::cast_from(0.12862205435420921092e-2_f64) * t16067 * t3117 * t42804 * t16020 + F::cast_from(0.13719685797782315831e-1_f64) * t11933 * t12017 + F::cast_from(0.13719685797782315831e-1_f64) * t11933 * t11871 + t1011 * t4915 * t41314 / F::new(8.0) + F::cast_from(0.11433071498151929859e-2_f64) * t42816 + F::cast_from(0.43445671692977333464e-1_f64) * t11268 * t3136;
    (t42804, t42820)
}
