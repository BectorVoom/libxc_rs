//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3101/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3101<F: Float>(t1042: F, t1261: F, t1264: F, t12712: F, t17351: F, t17353: F, t17644: F, t17654: F, t17693: F, t17694: F, t17696: F, t17799: F, t17800: F, t1797: F, t247: F, t3629: F, t44248: F, t44252: F, t44264: F, t44267: F, t44270: F, t44585: F, t5302: F, t54450: F, t56232: F, t56879: F, t56888: F, t56891: F, t56895: F, t56899: F, t56903: F, t56907: F) -> F {
    let t56932 = -F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t247 * t1264 * t56232 - F::cast_from(0.42874018118069736972e-3_f64) * t56879 * t17353 * t44585 * t3629 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t17353 * t12712 * t17644 + F::cast_from(0.14291339372689912324e-2_f64) * t56888 * t17696 + F::cast_from(0.14291339372689912324e-2_f64) * t17693 * t17694 * t56891 + F::cast_from(0.71456696863449561621e-3_f64) * t17693 * t17694 * t56895 + F::cast_from(0.71456696863449561621e-3_f64) * t17693 * t17694 * t56899 + F::cast_from(0.14291339372689912324e-2_f64) * t17654 * t17694 * t56903 - F::cast_from(0.71456696863449561621e-3_f64) * t17351 * t17694 * t56907 - F::cast_from(0.57165357490759649295e-3_f64) * t44248 + F::cast_from(0.19055119163586549765e-3_f64) * t44252 - F::cast_from(0.17149607247227894789e-2_f64) * t56888 * t17800 - F::cast_from(0.17149607247227894789e-2_f64) * t17693 * t17799 * t56891 - F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t17799 * t56895 - F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t17799 * t56899 + F::cast_from(0.19055119163586549765e-3_f64) * t44264 + F::cast_from(0.23818898954483187207e-3_f64) * t1261 * t1042 * t5302 * t54450 - F::cast_from(0.28582678745379824648e-3_f64) * t44270 + F::cast_from(0.21437009059034868486e-3_f64) * t44267 * t1797;
    t56932
}
