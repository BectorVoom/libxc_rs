//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1272/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1272(t24574: f64, t34241: f64, t27381: f64, t8866: f64, t7299: f64, t8054: f64, t1090: f64, t117840: f64, t118034: f64, t1186: f64, t14972: f64, t15820: f64, t1716: f64, t2128: f64, t2154: f64, t24589: f64, t24590: f64, t27549: f64, t27784: f64, t3242: f64, t3247: f64, t32493: f64, t32503: f64, t32523: f64, t34237: f64, t34322: f64, t3961: f64, t45349: f64, t4945: f64, t5059: f64, t7283: f64, t7302: f64, t8014: f64, t8060: f64, t8887: f64, t8888: f64, t8898: f64, t94369: f64, t94378: f64, t94514: f64) -> f64 {
    let t125206 = t24574 * t34241;
    let t125209 = t8866 * t27381;
    let t125218 = t7299 * t8054;
    let t125237 = -0.3289868133696452873e-1_f64 * t2128 * t24590 * t34322 + 0.10966227112321509577e-1_f64 * t24589 * t94369 * t2154 * t3247 * t3961 + 4.0_f64 * t4945 * t32493 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t32503 + 2.0_f64 * t15820 * t8888 + 2.0_f64 * t14972 * t8888 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t118034 - 0.16449340668482264365e-1_f64 * t7283 * t117840 * t8014 - 0.54831135561607547883e-2_f64 * t125206 - t15820 * t8898 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t125209 - t14972 * t8898 + 24.0_f64 * t27784 * t45349 * t8887 * t5059 - 0.16449340668482264365e-1_f64 * t7283 * t125218 * t7302 - 0.16449340668482264365e-1_f64 * t7283 * t1186 * t34237 - 0.10966227112321509577e-1_f64 * t24589 * t94378 * t8060 * t1090 - 0.54831135561607547883e-2_f64 * t24589 * t94514 * t32523 - 0.73108180748810063844e-2_f64 * t27549 * t94369 * t2154 * t3242 * t3961;
    t125237
}
