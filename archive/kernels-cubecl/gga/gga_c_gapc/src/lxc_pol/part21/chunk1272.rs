//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1272/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1272<F: Float>(t209: F, t35414: F, t35460: F, t35505: F, t35547: F, t35586: F, t35625: F, t35666: F, t35710: F, t10091: F, t1096: F, t11721: F, t12002: F, t13296: F, t15436: F, t24004: F, t24007: F, t2464: F, t2470: F, t34303: F, t34308: F, t34313: F, t35369: F, t35375: F, t35378: F, t3746: F, t7056: F) -> (F, F) {
    let t35714 = (t35414 + t35460 + t35505 + t35547 + t35586 + t35625 + t35666 + t35710) * t209;
    let t35717 = F::cast_from(24.0_f64) * t13296 * t2470 * t3746 - F::cast_from(12.0_f64) * t10091 * t24007 - F::cast_from(2.0_f64) * t1096 * t24004 + F::cast_from(8.0_f64) * t11721 * t7056 - F::cast_from(2.0_f64) * t12002 * t2464 + F::cast_from(2.0_f64) * t15436 * t3746 + t34303 - t34308 - t34313 + t35369 + t35375 - t35378 - t35714;
    (t35714, t35717)
}
