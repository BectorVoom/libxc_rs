//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1605/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605(t13100: f64, t828: f64, t12699: f64, t3624: f64, t1042: f64, t12257: f64, t12277: f64, t1261: f64, t1264: f64, t12705: f64, t12726: f64, t12777: f64, t12784: f64, t12787: f64, t12809: f64, t12810: f64, t12910: f64, t13065: f64, t16696: f64, t17459: f64, t17550: f64, t17688: f64, t247: f64, t3625: f64, t3626: f64, t3631: f64, t3711: f64, t3720: f64, t43180: f64, t43793: f64, t44191: f64, t44200: f64, t44202: f64, t44205: f64, t44215: f64, t5331: f64, t5340: f64, t5405: f64) -> f64 {
    let t44225 = t828 * t13100;
    let t44230 = t12699 * t3624;
    let t44239 = 0.25724410870841842184e-2_f64 * t12809 * t3720 * t12705 * t16696 + 0.28582678745379824648e-2_f64 * t5340 * t12787 * t12810 * t44191 - 0.14291339372689912324e-2_f64 * t5331 * t12787 * t12810 * t17688 - 0.22866142996303859718e-2_f64 * t44200 + 0.85748036236139473944e-3_f64 * t44202 * t13065 - 0.57165357490759649296e-2_f64 * t3711 * t1042 * t17550 * t44205 + 0.85748036236139473944e-2_f64 * t1261 * t1042 * t17550 * t43180 - 0.11433071498151929859e-2_f64 * t44215 - 0.34299214494455789578e-2_f64 * t1261 * t247 * t1264 * t43793 + 0.51448821741683684368e-2_f64 * t12910 * t3720 * t12726 * t17459 - 0.2540682555144873302e-2_f64 * t3625 * t44225 * t12257 * t5405 - 0.17149607247227894789e-2_f64 * t44230 * t3631 - 0.17149607247227894789e-2_f64 * t12784 * t12777 - 0.57165357490759649296e-3_f64 * t3625 * t3626 * t12277 * t5405;
    t44239
}
