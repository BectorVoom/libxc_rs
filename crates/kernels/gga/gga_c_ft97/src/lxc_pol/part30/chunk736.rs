//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 736/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk736<F: Float>(t33953: F, t798: F, t317: F, t193: F, t681: F, t7613: F, t1466: F, t6213: F, t7581: F, t7585: F, t880: F, t7586: F, t6208: F, t7150: F, t1491: F, t1774: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34260 = t798 * t33953;
    let t34261 = t34260 * t317;
    let t34262 = t193 * t34261;
    let t34265 = t681 * t7613;
    let t34267 = t1466 * t34265 / 18.0;
    let t34276 = t7581 * t6213 / 18.0;
    let t34277 = t7585 * t880;
    let t34278 = t193 * t34277;
    let t34281 = t681 * t7586;
    let t34283 = t1466 * t34281 / 9.0;
    let t34284 = t6208 * t7150;
    let t34287 = t1774 * t1491;
    (t34260, t34261, t34262, t34265, t34267, t34276, t34277, t34278, t34281, t34283, t34284, t34287)
}
