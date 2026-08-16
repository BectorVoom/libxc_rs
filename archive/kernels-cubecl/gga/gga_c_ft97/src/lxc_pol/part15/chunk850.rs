//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 850/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk850<F: Float>(t22479: F, t332: F, t113: F, t1259: F, t1275: F, t19920: F, t20489: F, t21802: F, t21806: F, t21812: F, t21815: F, t21818: F, t21900: F, t333: F, t4322: F, t4635: F, t5: F, t5430: F, t5475: F, t5480: F, t5483: F, t889: F, t992: F) -> (F, F) {
    let t22480 = t22479 * t332;
    let t22487 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t19920 * t1275 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t5 * t1259 * t4635 + t889 * t21802 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t21806 + t5 * t333 * t20489 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t21812 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t21815 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t21818 + t889 * t21900 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t4322 * t5480 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t4322 * t5475 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4322 * t5483 + t5 * t22480 * t113 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t5 * t5430 * t992;
    (t22480, t22487)
}
