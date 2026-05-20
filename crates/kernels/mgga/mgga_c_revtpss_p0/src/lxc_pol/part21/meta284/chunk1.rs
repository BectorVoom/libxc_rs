//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1519/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1519<F: Float>(t10246: F, t2362: F, t10236: F, t108: F, t101: F, t10217: F, t10229: F, t10233: F, t10237: F, t10243: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F) -> (F, F, F, F) {
    let t10247 = t10246 * t2362;
    let t10250 = -t10236;
    let t10251 = t108 * t10250;
    let t10254 = -F::new(440.0) / F::new(27.0) * t10217 * t101 + F::new(200.0) / F::new(9.0) * t2344 * t659 - F::new(50.0) / F::new(9.0) * t656 * t2351 - F::new(25.0) / F::new(3.0) * t656 * t2354 - F::new(10.0) / F::new(27.0) * t97 * t10229 + F::new(10.0) / F::new(3.0) * t97 * t10233 + F::new(5.0) / F::new(3.0) * t97 * t10237 - F::new(10.0) / F::new(27.0) * t105 * t10243 + F::new(10.0) / F::new(3.0) * t105 * t10247 + F::new(5.0) / F::new(3.0) * t105 * t10251;
    (t10247, t10250, t10251, t10254)
}
