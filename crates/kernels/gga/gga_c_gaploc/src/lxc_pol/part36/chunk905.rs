//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 905/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk905<F: Float>(t123: F, t31730: F, t2326: F, t9074: F, t12797: F, t1358: F, t12773: F, t6305: F, t2268: F, t42212: F, t888: F, t1365: F, t42408: F, t42625: F, t42629: F, t42633: F, t42637: F, t42638: F, t42641: F, t42645: F, t42648: F, t42652: F, t42655: F, t42659: F, t42661: F, t42664: F) -> F {
    let t42669 = t31730 * t123;
    let t42671 = t9074 * t42669 * t2326;
    let t42673 = t1358 * t12797;
    let t42674 = F::cast_from(0.31616674039640166221e-2_f64) * t42673;
    let t42675 = t6305 * t12773;
    let t42678 = t2268 * t42212 * t888;
    let t42680 = -F::cast_from(0.1138200265427045984e0_f64) * t42625 - t42629 - t42633 + t42637 - t42638 + t42641 - t42645 + t42648 - t42652 + t42655 - t42659 - F::cast_from(0.23712505529730124666e-2_f64) * t42661 + F::cast_from(0.23712505529730124666e-2_f64) * t42664 + F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t1365 * t42408 - F::cast_from(0.71137516589190373998e-2_f64) * t42671 - t42674 - F::cast_from(0.1707300398140568976e0_f64) * t42675 - F::cast_from(0.1707300398140568976e0_f64) * t42678;
    t42680
}
