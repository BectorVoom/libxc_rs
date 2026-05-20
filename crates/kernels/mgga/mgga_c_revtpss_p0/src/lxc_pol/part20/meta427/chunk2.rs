//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1605/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605<F: Float>(t13100: F, t828: F, t12699: F, t3624: F, t1042: F, t12257: F, t12277: F, t1261: F, t1264: F, t12705: F, t12726: F, t12777: F, t12784: F, t12787: F, t12809: F, t12810: F, t12910: F, t13065: F, t16696: F, t17459: F, t17550: F, t17688: F, t247: F, t3625: F, t3626: F, t3631: F, t3711: F, t3720: F, t43180: F, t43793: F, t44191: F, t44200: F, t44202: F, t44205: F, t44215: F, t5331: F, t5340: F, t5405: F) -> F {
    let t44225 = t828 * t13100;
    let t44230 = t12699 * t3624;
    let t44239 = F::cast_from(0.25724410870841842184e-2_f64) * t12809 * t3720 * t12705 * t16696 + F::cast_from(0.28582678745379824648e-2_f64) * t5340 * t12787 * t12810 * t44191 - F::cast_from(0.14291339372689912324e-2_f64) * t5331 * t12787 * t12810 * t17688 - F::cast_from(0.22866142996303859718e-2_f64) * t44200 + F::cast_from(0.85748036236139473944e-3_f64) * t44202 * t13065 - F::cast_from(0.57165357490759649296e-2_f64) * t3711 * t1042 * t17550 * t44205 + F::cast_from(0.85748036236139473944e-2_f64) * t1261 * t1042 * t17550 * t43180 - F::cast_from(0.11433071498151929859e-2_f64) * t44215 - F::cast_from(0.34299214494455789578e-2_f64) * t1261 * t247 * t1264 * t43793 + F::cast_from(0.51448821741683684368e-2_f64) * t12910 * t3720 * t12726 * t17459 - F::cast_from(0.2540682555144873302e-2_f64) * t3625 * t44225 * t12257 * t5405 - F::cast_from(0.17149607247227894789e-2_f64) * t44230 * t3631 - F::cast_from(0.17149607247227894789e-2_f64) * t12784 * t12777 - F::cast_from(0.57165357490759649296e-3_f64) * t3625 * t3626 * t12277 * t5405;
    t44239
}
