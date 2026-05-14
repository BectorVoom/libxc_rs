//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1413/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1413<F: Float>(t1604: F, t32697: F, t10084: F, t2214: F, t514: F, t20871: F, t785: F, t788: F, t9981: F, t20947: F, t9937: F, t2207: F, t2837: F, t9418: F, t2139: F, t30312: F, t30315: F, t30318: F, t30322: F, t30333: F, t30339: F, t30342: F, t3056: F, t33815: F, t360: F, t5109: F, t7977: F) -> (F,) {
    let t34270 = t1604 * t32697;
    let t34273 = t514 * t2214 * t10084;
    let t34277 = t20871 * t785 * t788 * t9981;
    let t34281 = t20947 * t785 * t788 * t9937;
    let t34284 = t2207 * t2837 * t9418;
    let t34290 = -0.40752780427737692339e0 * t30312 + 0.49390868872016336988e-1 * t30315 - 0.49390868872016336989e-1 * t30318 + 0.16463622957338778997e-1 * t30322 + 0.39006997830244208535e0 * t2139 * t5109 * t33815 + 0.34930954652346593433e-1 * t30333 - 0.1047928639570397803e0 * t30339 - 0.4191714558281591212e0 * t30342 + 0.16463622957338778997e-1 * t34270 - 0.97574405393827830187e-2 * t34273 - 0.34930954652346593435e-1 * t34277 - 0.34930954652346593435e0 * t34281 - 0.52396431978519890152e-1 * t34284 + 0.39006997830244208535e0 * t2139 * t360 * t7977 * t3056;
    (t34290,)
}
