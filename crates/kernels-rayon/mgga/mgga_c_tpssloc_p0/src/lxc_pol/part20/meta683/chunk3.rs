//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2587/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587(t3030: f64, t4940: f64, t3623: f64, t1009: f64, t15425: f64, t1243: f64, t50816: f64, t50818: f64, t50821: f64, t51111: f64, t51113: f64, t51119: f64, t51122: f64, t51124: f64, t51126: f64, t51128: f64, t51131: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64, t51793: f64, t51795: f64, t51797: f64, t51800: f64, t51802: f64) -> (f64, f64, f64, f64, f64) {
    let t52434 = t4940 * t3030;
    let t52435 = t52434 * t3623;
    let t52446 = t15425 * t1009;
    let t52447 = t52446 * t1243;
    let t52450 = -t50816 - t50818 - t50821 - t51111 - t51113 + t51119 + t51122 + t51124 + t51126 + t51128 - t51131 + t51133 + t51245 - t51248 - t51251 + t51793 - t51795 - t51797 - t51800 + t51802;
    (t52434, t52435, t52446, t52447, t52450)
}
