//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2587/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587<F: Float>(t3030: F, t4940: F, t3623: F, t1009: F, t15425: F, t1243: F, t50816: F, t50818: F, t50821: F, t51111: F, t51113: F, t51119: F, t51122: F, t51124: F, t51126: F, t51128: F, t51131: F, t51133: F, t51245: F, t51248: F, t51251: F, t51793: F, t51795: F, t51797: F, t51800: F, t51802: F) -> (F, F, F, F, F) {
    let t52434 = t4940 * t3030;
    let t52435 = t52434 * t3623;
    let t52446 = t15425 * t1009;
    let t52447 = t52446 * t1243;
    let t52450 = -t50816 - t50818 - t50821 - t51111 - t51113 + t51119 + t51122 + t51124 + t51126 + t51128 - t51131 + t51133 + t51245 - t51248 - t51251 + t51793 - t51795 - t51797 - t51800 + t51802;
    (t52434, t52435, t52446, t52447, t52450)
}
