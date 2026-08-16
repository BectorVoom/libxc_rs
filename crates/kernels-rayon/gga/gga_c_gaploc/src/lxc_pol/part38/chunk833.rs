//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 833/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk833(t13319: f64, t6305: f64, t13296: f64, t555: f64, t13313: f64, t10216: f64, t1063: f64, t1064: f64, t13250: f64, t2268: f64, t2765: f64, t3358: f64, t42584: f64, t44305: f64, t44306: f64, t44309: f64, t44313: f64, t44316: f64, t44319: f64, t44322: f64, t44325: f64, t44328: f64, t44329: f64, t44334: f64, t494: f64, t7930: f64) -> (f64, f64) {
    let t44336 = 0.28455006635676149599e-1_f64 * t6305 * t13319;
    let t44337 = t555 * t13296;
    let t44350 = 0.85365019907028448797e-1_f64 * t6305 * t13313;
    let t44351 = 0.94850022118920498664e-2_f64 * t42584 - t44305 - t44306 - t44309 + t44313 + t44316 - t44319 + t44322 - t44325 + t44328 + 0.28455006635676149599e-1_f64 * t1063 * t1064 * t44329 + t44334 + t44336 - 0.85365019907028448797e-1_f64 * t2268 * t44337 * t494 - 0.17073003981405689759e0_f64 * t2268 * t2765 * t10216 - 0.17073003981405689759e0_f64 * t6305 * t13250 - 0.17073003981405689759e0_f64 * t2268 * t7930 * t3358 - t44350;
    (t44337, t44351)
}
