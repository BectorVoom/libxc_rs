//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3545/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3545<F: Float>(t11922: F, t20069: F, t4899: F, t1045: F, t11704: F, t11774: F, t11859: F, t11875: F, t11933: F, t15690: F, t15702: F, t15782: F, t15917: F, t15962: F, t16043: F, t16049: F, t16170: F, t19501: F, t19718: F, t19738: F, t19745: F, t19750: F, t19979: F, t20070: F, t2852: F, t2857: F, t3115: F, t3117: F, t3155: F, t372: F, t4823: F, t53553: F, t65186: F, t65876: F, t66341: F, t73: F) -> F {
    let t67426 = t4899 * t11922 * t20069;
    let t67430 = F::cast_from(0.22866142996303859718e-2_f64) * t11933 * t19745 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t65186 * t1045 - F::cast_from(0.42874018118069736972e-3_f64) * t11859 * t3117 * t66341 * t3155 + F::cast_from(0.11433071498151929859e-2_f64) * t11774 * t372 * t15690 * t73 * t15962 * t65876 + F::cast_from(0.11433071498151929859e-2_f64) * t11774 * t372 * t4823 * t2857 * t15702 - F::cast_from(0.95275595817932748826e-3_f64) * t11774 * t372 * t16170 * t2852 * t15702 - F::cast_from(0.95275595817932748826e-3_f64) * t11774 * t372 * t19979 * t73 * t11704 * t65876 - F::cast_from(0.13719685797782315831e-1_f64) * t53553 * t19750 - F::cast_from(0.85748036236139473944e-3_f64) * t15917 * t19718 + F::cast_from(0.21437009059034868486e-3_f64) * t11875 * t3117 * t19501 * t16043 + F::cast_from(0.22866142996303859718e-2_f64) * t16049 * t20070 - F::cast_from(0.28582678745379824648e-3_f64) * t67426 + F::cast_from(0.17149607247227894789e-2_f64) * t19738 * t15782;
    t67430
}
