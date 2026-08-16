//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1540/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1540<F: Float>(t11671: F, t11926: F, t1045: F, t2862: F, t999: F, t3075: F, t606: F, t1042: F, t1063: F, t11678: F, t11774: F, t11930: F, t15785: F, t15906: F, t16043: F, t16081: F, t16089: F, t16101: F, t16199: F, t3092: F, t3115: F, t3117: F, t42309: F, t42804: F, t43180: F, t43277: F, t43279: F, t43285: F, t43288: F, t43291: F, t43292: F) -> F {
    let t43297 = t11926 * t11671;
    let t43301 = t1045 * t2862 * t999;
    let t43313 = t606 * t3075;
    let t43318 = -F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t1042 * t16199 * t43180 - F::cast_from(0.17149607247227894789e-2_f64) * t43277 + F::cast_from(0.77173232612525526552e-2_f64) * t16081 * t3117 * t42804 * t43279 + F::cast_from(0.51448821741683684368e-2_f64) * t43285 * t11930 - F::cast_from(0.34299214494455789577e-2_f64) * t43288 - F::cast_from(0.51448821741683684368e-2_f64) * t43291 * t3117 * t43292 * t1045 - F::cast_from(0.27439371595564631662e-1_f64) * t43297 * t11930 - F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t42309 * t43301 - F::cast_from(0.77173232612525526552e-2_f64) * t15906 * t3117 * t42804 * t15785 - F::cast_from(0.12862205435420921092e-2_f64) * t3115 * t3117 * t11678 * t16043 + F::cast_from(0.34299214494455789578e-2_f64) * t16089 * t3092 * t16101 * t43313;
    t43318
}
