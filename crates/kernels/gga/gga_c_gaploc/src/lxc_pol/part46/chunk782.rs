//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 782/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk782<F: Float>(t2268: F, t41596: F, t426: F, t535: F, t39671: F, t39674: F, t39677: F, t39679: F, t39681: F, t12837: F, t6305: F, t2440: F, t3340: F, t10223: F, t894: F, t1063: F, t12814: F, t12971: F, t42580: F, t42582: F, t42584: F, t42588: F, t42591: F, t42594: F, t42597: F, t448: F) -> (F,) {
    let t42601 = 0.28455006635676149599e-1 * t2268 * t535 * t41596 * t426;
    let t42602 = 0.284550066356761496e-1 * t39671;
    let t42603 = 0.142275033178380748e-1 * t39674;
    let t42604 = 0.23712505529730124666e-2 * t39677;
    let t42605 = 0.47425011059460249332e-2 * t39679;
    let t42606 = 0.71137516589190373998e-2 * t39681;
    let t42607 = t6305 * t12837;
    let t42610 = t2268 * t2440 * t3340;
    let t42613 = t2268 * t894 * t10223;
    let t42621 = -t42580 + t42582 + 0.47425011059460249332e-2 * t42584 - t42588 - t42591 - t42594 + t42597 + t42601 + t42602 - t42603 + t42604 + t42605 - t42606 + 0.56910013271352299198e-1 * t42607 + 0.56910013271352299198e-1 * t42610 + 0.56910013271352299198e-1 * t42613 - 0.28455006635676149599e-1 * t1063 * t12971 * t448 + 0.28455006635676149599e-1 * t2268 * t535 * t12814;
    (t42621,)
}
