//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3545/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3545(t11922: f64, t20069: f64, t4899: f64, t1045: f64, t11704: f64, t11774: f64, t11859: f64, t11875: f64, t11933: f64, t15690: f64, t15702: f64, t15782: f64, t15917: f64, t15962: f64, t16043: f64, t16049: f64, t16170: f64, t19501: f64, t19718: f64, t19738: f64, t19745: f64, t19750: f64, t19979: f64, t20070: f64, t2852: f64, t2857: f64, t3115: f64, t3117: f64, t3155: f64, t372: f64, t4823: f64, t53553: f64, t65186: f64, t65876: f64, t66341: f64, t73: f64) -> f64 {
    let t67426 = t4899 * t11922 * t20069;
    let t67430 = 0.22866142996303859718e-2_f64 * t11933 * t19745 - 0.21437009059034868486e-3_f64 * t3115 * t3117 * t65186 * t1045 - 0.42874018118069736972e-3_f64 * t11859 * t3117 * t66341 * t3155 + 0.11433071498151929859e-2_f64 * t11774 * t372 * t15690 * t73 * t15962 * t65876 + 0.11433071498151929859e-2_f64 * t11774 * t372 * t4823 * t2857 * t15702 - 0.95275595817932748826e-3_f64 * t11774 * t372 * t16170 * t2852 * t15702 - 0.95275595817932748826e-3_f64 * t11774 * t372 * t19979 * t73 * t11704 * t65876 - 0.13719685797782315831e-1_f64 * t53553 * t19750 - 0.85748036236139473944e-3_f64 * t15917 * t19718 + 0.21437009059034868486e-3_f64 * t11875 * t3117 * t19501 * t16043 + 0.22866142996303859718e-2_f64 * t16049 * t20070 - 0.28582678745379824648e-3_f64 * t67426 + 0.17149607247227894789e-2_f64 * t19738 * t15782;
    t67430
}
