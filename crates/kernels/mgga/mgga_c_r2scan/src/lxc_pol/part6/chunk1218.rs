//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1218/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1218<F: Float>(t1762: F, t5206: F, t5559: F, t1763: F, t5534: F, t1767: F, t5556: F, t1376: F, t2035: F, t41: F, t4885: F, t726: F, t1743: F, t1835: F, t1837: F, t234: F) -> (F, F, F, F, F, F) {
    let t22481 = 0.23116671906084754117e2 * t1762 * t5206 * t5559;
    let t22484 = 0.1301229756036208781e0 * t1762 * t1763 * t5534;
    let t22487 = 0.39036892681086263432e0 * t1762 * t1767 * t5556;
    let t22489 = t41 * t1376 * t2035;
    let t22491 = t4885 * t726;
    let t22496 = 0.62337092780453269531e3 * t234 * t1835 * t1743 * t1837;
    (t22481, t22484, t22487, t22489, t22491, t22496)
}
