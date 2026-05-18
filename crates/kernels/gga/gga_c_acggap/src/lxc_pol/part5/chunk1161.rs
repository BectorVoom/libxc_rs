//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1161/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1161<F: Float>(t1188: F, t1410: F, t322: F, t5853: F, t1165: F, t13585: F, t5852: F, t1173: F, t1180: F, t1532: F, t1552: F, t15930: F, t15932: F, t15934: F, t15936: F, t15938: F, t15945: F, t15947: F, t1748: F, t18834: F, t301: F, t3403: F, t5606: F, t5984: F, t6258: F, t839: F) -> (F, F) {
    let t20935 = t1188 * t1410;
    let t20944 = t5853 * t322;
    let t20947 = t13585 * t1165 * t5852 * t20944;
    let t20959 = -F::new(0.17149607247227894789e-1) * t3403 * t1165 * t1532 * t6258 * t301 - F::new(0.85748036236139473945e-2) * t3403 * t1165 * t1532 * t1748 * t839 + F::new(0.17149607247227894789e-2) * t1180 * t1165 * t1552 * t20935 - F::new(0.51448821741683684366e-2) * t1180 * t1165 * t18834 * t5984 + F::new(0.25724410870841842184e-2) * t20947 - F::new(0.15117061203111996147e0) * t15930 - F::new(0.40015750243531754508e-2) * t15932 - F::new(0.80031500487063509016e-2) * t15934 - F::new(0.40015750243531754508e-2) * t15936 - F::new(0.16006300097412701803e-1) * t15938 - F::new(0.68598428988911579156e-2) * t15945 - F::new(0.68598428988911579156e-2) * t1173 * t1165 * t15947 * t5606;
    (t20944, t20959)
}
