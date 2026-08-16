//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 678/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk678(t1249: f64, t3898: f64, t2381: f64, t1220: f64, t1224: f64, t1230: f64, t1238: f64, t1242: f64, t2346: f64, t2367: f64, t2380: f64, t2395: f64, t2404: f64, t3165: f64, t3172: f64, t3193: f64, t3214: f64, t3217: f64, t3230: f64, t3846: f64, t3849: f64, t385: f64, t3860: f64, t3866: f64, t3870: f64, t3877: f64, t388: f64, t3883: f64, t3887: f64, t3892: f64, t404: f64, t407: f64, t918: f64) -> (f64, f64, f64) {
    let t3899 = t1249 * t3898;
    let t3900 = t2381 * t3899;
    let t3903 = -t3172 / 144.0_f64 - t385 * t3846 / 96.0_f64 + 11.0_f64 / 108.0_f64 * t3849 * t388 + t1220 * t1224 / 18.0_f64 - 0.15244095330869239812e-2_f64 * t3217 - t2346 + 0.72409452821628889107e-2_f64 * t3860 * t407 + 0.45732285992607719436e-2_f64 * t1238 * t1242 - 0.42874018118069736972e-3_f64 * t404 * t3866 + 0.12862205435420921092e-2_f64 * t404 * t3870 - 0.57165357490759649296e-3_f64 * t3230 - 0.21437009059034868486e-3_f64 * t2395 * t3877 + 0.21437009059034868486e-3_f64 * t918 * t3883 + 0.42874018118069736972e-3_f64 * t2367 * t3887 - t3165 / 54.0_f64 - t2404 + t385 * t3892 / 48.0_f64 + 0.28582678745379824648e-3_f64 * t3193 - 0.22866142996303859718e-2_f64 * t3214 * t1230 - 0.85748036236139473944e-3_f64 * t2380 * t3900;
    (t3899, t3900, t3903)
}
