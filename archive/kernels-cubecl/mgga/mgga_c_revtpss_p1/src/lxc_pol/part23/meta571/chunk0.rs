//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2159/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2159<F: Float>(t1390: F, t22954: F, t828: F, t1388: F, t13959: F, t14013: F, t14043: F, t22156: F, t22179: F, t22183: F, t22260: F, t22264: F, t22268: F, t22285: F, t22292: F, t22914: F, t9953: F) -> (F, F) {
    let t22956 = t1390 * t828 * t22954;
    let t22962 = -F::cast_from(0.17006693853500995666e-1_f64) * t13959 - F::cast_from(0.24009450146119052704e-1_f64) * t22156 - F::cast_from(0.5421477899694558815e-4_f64) * t14013 + F::cast_from(0.30011812682648815881e-2_f64) * t22179 + F::cast_from(0.76230004213927992337e-4_f64) * t22183 - F::cast_from(0.38115002106963996168e-4_f64) * t22260 - F::cast_from(0.17149607247227894789e-3_f64) * t22264 - F::cast_from(0.38115002106963996168e-4_f64) * t22268 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t22914 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t22956 - t9953 - F::cast_from(0.60023625365297631762e-2_f64) * t22285 + F::cast_from(0.30011812682648815881e-2_f64) * t22292 + F::cast_from(0.40656002247428262579e-3_f64) * t14043;
    (t22956, t22962)
}
