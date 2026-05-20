//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1782/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1782<F: Float>(t14230: F, t28911: F, t26304: F, t27972: F, t27864: F, t1445: F, t1904: F, t2027: F, t213: F, t25930: F, t26282: F, t26365: F, t26366: F, t27868: F, t28863: F, t28890: F, t28895: F, t28897: F, t28899: F, t28903: F, t28905: F, t28909: F, t561: F, t5775: F, t7295: F, t7511: F) -> (F, F, F, F) {
    let t28912 = t28911 * t14230;
    let t28915 = t26304 * t27972;
    let t28918 = t26304 * t27864;
    let t28923 = F::cast_from(0.8673628188205199462e0_f64) * t7295 * t28863 - F::cast_from(0.65854491829355115987e0_f64) * t26282 * t1904 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t28890 - t26365 + F::cast_from(0.72280234901709995518e-2_f64) * t26366 - F::cast_from(0.72280234901709995518e-2_f64) * t28895 + F::cast_from(0.12851425765524037203e-1_f64) * t28897 - F::cast_from(0.65854491829355115987e0_f64) * t28899 * t1445 + F::cast_from(0.54878743191129263322e-2_f64) * t28903 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t28905 * t561 + F::cast_from(0.72280234901709995518e-2_f64) * t28909 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t28912 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t28915 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t28918 - F::cast_from(0.65854491829355115987e0_f64) * t7511 * t5775;
    (t28912, t28915, t28918, t28923)
}
