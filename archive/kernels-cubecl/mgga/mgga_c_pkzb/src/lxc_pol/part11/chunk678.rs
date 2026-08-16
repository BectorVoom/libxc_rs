//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 678/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk678<F: Float>(t1249: F, t3898: F, t2381: F, t1220: F, t1224: F, t1230: F, t1238: F, t1242: F, t2346: F, t2367: F, t2380: F, t2395: F, t2404: F, t3165: F, t3172: F, t3193: F, t3214: F, t3217: F, t3230: F, t3846: F, t3849: F, t385: F, t3860: F, t3866: F, t3870: F, t3877: F, t388: F, t3883: F, t3887: F, t3892: F, t404: F, t407: F, t918: F) -> (F, F, F) {
    let t3899 = t1249 * t3898;
    let t3900 = t2381 * t3899;
    let t3903 = -t3172 / F::cast_from(144.0_f64) - t385 * t3846 / F::cast_from(96.0_f64) + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t3849 * t388 + t1220 * t1224 / F::cast_from(18.0_f64) - F::cast_from(0.15244095330869239812e-2_f64) * t3217 - t2346 + F::cast_from(0.72409452821628889107e-2_f64) * t3860 * t407 + F::cast_from(0.45732285992607719436e-2_f64) * t1238 * t1242 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t3866 + F::cast_from(0.12862205435420921092e-2_f64) * t404 * t3870 - F::cast_from(0.57165357490759649296e-3_f64) * t3230 - F::cast_from(0.21437009059034868486e-3_f64) * t2395 * t3877 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t3883 + F::cast_from(0.42874018118069736972e-3_f64) * t2367 * t3887 - t3165 / F::cast_from(54.0_f64) - t2404 + t385 * t3892 / F::cast_from(48.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t3193 - F::cast_from(0.22866142996303859718e-2_f64) * t3214 * t1230 - F::cast_from(0.85748036236139473944e-3_f64) * t2380 * t3900;
    (t3899, t3900, t3903)
}
