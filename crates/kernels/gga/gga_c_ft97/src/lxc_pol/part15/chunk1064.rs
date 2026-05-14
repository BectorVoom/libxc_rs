//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1064/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1064<F: Float>(t113: F, t1259: F, t1274: F, t20489: F, t21801: F, t21802: F, t21806: F, t21812: F, t21815: F, t21899: F, t22480: F, t332: F, t333: F, t4322: F, t4635: F, t5: F, t5430: F, t86571: F, t889: F, t91145: F, t91216: F, t91269: F, t91334: F, t91387: F, t91423: F, t992: F) -> (F,) {
    let t91432 = t5 * (t91145 + t91216) * t332 * t113 / 4.0 + t5 * t22480 * t992 + t5 * t333 * t86571 / 4.0 + 3.0 / 2.0 * t5 * t5430 * t4635 + t889 * t21801 * t992 + 3.0 * t4322 * t21812 + t4322 * t21802 + t5 * t1259 * t20489 + 3.0 * t4322 * t21815 + t889 * t1274 * t20489 + 3.0 * t4322 * t21806 + t889 * (t91269 + t91334 + t91387 + t91423) * t332 * t113 / 4.0 + t889 * t21899 * t992;
    (t91432,)
}
