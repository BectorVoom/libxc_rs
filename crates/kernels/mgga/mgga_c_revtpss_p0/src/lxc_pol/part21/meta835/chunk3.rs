//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3131/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3131<F: Float>(t57726: F, t1042: F, t1248: F, t12784: F, t12805: F, t12809: F, t12812: F, t12855: F, t12858: F, t12872: F, t12910: F, t13076: F, t16775: F, t1715: F, t17396: F, t17500: F, t17514: F, t17674: F, t17677: F, t17682: F, t21014: F, t3372: F, t3604: F, t3611: F, t3625: F, t3626: F, t3711: F, t372: F, t3720: F, t44431: F, t44521: F, t44634: F, t44637: F, t471: F, t5056: F, t5274: F, t5277: F, t5331: F, t5340: F, t5405: F, t57687: F, t57689: F, t57696: F, t57707: F, t57710: F) -> F {
    let t57727 = t57726 / F::cast_from(324.0_f64);
    let t57728 = -F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t3626 * t5056 * t17677 + F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t3626 * t5056 * t17682 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t1715 * t44431 * t471 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t16775 * t5405 + F::cast_from(0.57165357490759649295e-3_f64) * t44634 - F::cast_from(5.0_f64) / F::cast_from(3888.0_f64) * t57687 - F::cast_from(0.2540682555144873302e-2_f64) * t57689 - F::cast_from(0.17149607247227894789e-2_f64) * t44521 * t372 * t5277 * t1248 * t17514 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t57696 * t3604 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t57696 * t3611 + F::cast_from(0.34299214494455789577e-2_f64) * t17396 * t12805 + F::cast_from(0.68598428988911579154e-2_f64) * t57707 * t12858 - F::cast_from(0.34299214494455789577e-2_f64) * t57710 * t12812 + F::cast_from(0.21437009059034868486e-3_f64) * t5274 * t13076 + F::cast_from(0.42874018118069736972e-3_f64) * t44637 - F::cast_from(0.42874018118069736972e-3_f64) * t12784 * t17674 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t17500 * t3372 - F::cast_from(0.68598428988911579154e-2_f64) * t21014 * t12872 - t57727;
    t57728
}
