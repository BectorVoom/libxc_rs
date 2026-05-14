//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 727/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk727<F: Float>(t44324: F, t2268: F, t2440: F, t3518: F, t44268: F, t447: F, t13319: F, t6313: F, t6305: F, t13296: F, t555: F, t13313: F, t10216: F, t1063: F, t1064: F, t13250: F, t2765: F, t3358: F, t42584: F, t44305: F, t44306: F, t44309: F, t44313: F, t44316: F, t44319: F, t44322: F, t494: F, t7930: F) -> (F, F, F) {
    let t44325 = 0.82993769354055436331e-2 * t44324;
    let t44328 = 0.28455006635676149599e-1 * t2268 * t2440 * t3518;
    let t44329 = t44268 * t447;
    let t44334 = 0.37940008847568199465e-1 * t6313 * t13319;
    let t44336 = 0.28455006635676149599e-1 * t6305 * t13319;
    let t44337 = t555 * t13296;
    let t44350 = 0.85365019907028448797e-1 * t6305 * t13313;
    let t44351 = 0.94850022118920498664e-2 * t42584 - t44305 - t44306 - t44309 + t44313 + t44316 - t44319 + t44322 - t44325 + t44328 + 0.28455006635676149599e-1 * t1063 * t1064 * t44329 + t44334 + t44336 - 0.85365019907028448797e-1 * t2268 * t44337 * t494 - 0.17073003981405689759e0 * t2268 * t2765 * t10216 - 0.17073003981405689759e0 * t6305 * t13250 - 0.17073003981405689759e0 * t2268 * t7930 * t3358 - t44350;
    (t44329, t44337, t44351)
}
