//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3529/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3529<F: Float>(t1045: F, t11774: F, t11852: F, t15140: F, t15614: F, t15692: F, t15696: F, t15700: F, t16229: F, t1668: F, t19986: F, t372: F, t42328: F, t42907: F, t43082: F, t53914: F, t54667: F, t54678: F, t54680: F, t54687: F, t54693: F, t54704: F, t54708: F, t66689: F) -> F {
    let t66893 = F::cast_from(0.1270341277572436651e-2_f64) * t15700 * t372 * t11852 * t1668 * t1045 * t15140 - F::cast_from(0.57165357490759649296e-3_f64) * t53914 * t19986 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t15696 * t15614 + F::cast_from(0.6351706387862183255e-3_f64) * t54667 + F::cast_from(0.30488190661738479624e-2_f64) * t54678 - F::cast_from(0.57165357490759649296e-3_f64) * t54680 - F::cast_from(0.1270341277572436651e-3_f64) * t54687 - F::cast_from(0.28582678745379824648e-3_f64) * t54693 - F::cast_from(0.57165357490759649296e-3_f64) * t54704 + F::cast_from(0.19055119163586549765e-3_f64) * t54708 - F::cast_from(0.6351706387862183255e-4_f64) * t42907 - F::cast_from(0.11433071498151929859e-2_f64) * t43082 * t66689 * t16229 + F::cast_from(0.57165357490759649296e-3_f64) * t42328 * t66689 * t15692;
    t66893
}
