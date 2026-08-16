//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1016/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1016<F: Float>(t26431: F, t26470: F, t1378: F, t7696: F, t794: F, t6897: F, t225: F, t7704: F, t1385: F, t7749: F, t3887: F, t1375: F, t1386: F, t16022: F, t16030: F, t1843: F, t2016: F, t22670: F, t22676: F, t26366: F, t26371: F, t3758: F, t3882: F, t5326: F, t6958: F, t7750: F) -> (F, F, F, F, F, F) {
    let t26471 = t26431 + t26470;
    let t26472 = t1378 * t26471;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26481 = t7749 * t1385;
    let t26482 = t3887 * t26481;
    let t26485 = -t26366 * t1386 + F::cast_from(2.0_f64) * t6958 * t5326 + F::cast_from(2.0_f64) * t1375 * t26371 - t3882 * t7750 - t22670 * t1843 - t16030 * t2016 - t16022 * t2016 - t1375 * t26472 - F::cast_from(0.41123351671205660912e-2_f64) * t26475 - t26477 * t1386 + F::cast_from(0.41123351671205660912e-2_f64) * t22676 - t3758 * t7750 + F::cast_from(2.0_f64) * t1375 * t26482;
    (t26471, t26472, t26477, t26481, t26482, t26485)
}
