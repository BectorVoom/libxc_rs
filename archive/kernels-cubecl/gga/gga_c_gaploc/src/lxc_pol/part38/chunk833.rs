//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 833/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk833<F: Float>(t13319: F, t6305: F, t13296: F, t555: F, t13313: F, t10216: F, t1063: F, t1064: F, t13250: F, t2268: F, t2765: F, t3358: F, t42584: F, t44305: F, t44306: F, t44309: F, t44313: F, t44316: F, t44319: F, t44322: F, t44325: F, t44328: F, t44329: F, t44334: F, t494: F, t7930: F) -> (F, F) {
    let t44336 = F::cast_from(0.28455006635676149599e-1_f64) * t6305 * t13319;
    let t44337 = t555 * t13296;
    let t44350 = F::cast_from(0.85365019907028448797e-1_f64) * t6305 * t13313;
    let t44351 = F::cast_from(0.94850022118920498664e-2_f64) * t42584 - t44305 - t44306 - t44309 + t44313 + t44316 - t44319 + t44322 - t44325 + t44328 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t1064 * t44329 + t44334 + t44336 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t44337 * t494 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t2765 * t10216 - F::cast_from(0.17073003981405689759e0_f64) * t6305 * t13250 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t7930 * t3358 - t44350;
    (t44337, t44351)
}
