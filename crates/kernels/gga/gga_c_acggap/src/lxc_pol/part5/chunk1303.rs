//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1303/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1303<F: Float>(t14173: F, t6144: F, t1181: F, t3361: F, t4623: F, t530: F, t3382: F, t5796: F, t1140: F, t5586: F, t1017: F, t1165: F, t13299: F, t13364: F, t17185: F, t1849: F, t18768: F, t18770: F, t18772: F, t20305: F, t22705: F, t336: F, t3396: F, t360: F, t367: F, t372: F, t4261: F, t4762: F, t6138: F, t8790: F, t8927: F) -> F {
    let t24211 = t14173 * t6144;
    let t24218 = t3361 * t1181 * t530 * t4623;
    let t24220 = t3382 * t5796;
    let t24222 = t1140 * t5586;
    let t24242 = -F::cast_from(0.20579528696673473746e-1_f64) * t3396 * t1165 * t6138 * t4762 + F::cast_from(0.34299214494455789578e-2_f64) * t24211 + F::cast_from(0.45351183609335988443e-1_f64) * t18768 + F::cast_from(0.90702367218671976884e-1_f64) * t18770 - F::cast_from(0.80031500487063509015e-2_f64) * t18772 + F::cast_from(0.68598428988911579156e-2_f64) * t24218 - F::cast_from(0.17149607247227894789e-2_f64) * t24220 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t24222 + t367 * t336 * t22705 * t1017 / F::cast_from(48.0_f64) + t4261 * t8927 * t1849 * t1017 / F::cast_from(4.0_f64) - F::cast_from(0.13719685797782315831e-1_f64) * t17185 * t13364 * t8790 * t20305 * t360 + F::cast_from(0.13719685797782315831e-1_f64) * t17185 * t13299 * t8790 * t20305 * t372;
    t24242
}
