//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1442/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1442<F: Float>(t122423: F, t122438: F, t1799: F, t2085: F, t1307: F, t26331: F, t26446: F, t1992: F, t550: F, t6976: F, t93501: F, t22704: F, t22705: F, t33280: F) -> (F, F, F, F, F) {
    let t122439 = t122423 + t122438;
    let t122448 = t2085 * t1799;
    let t122451 = t26331 * t26446 * t122448 * t1307;
    let t122457 = t1992 * t6976 * t93501 * t550;
    let t122460 = t22704 * t22705 * t33280;
    (t122439, t122448, t122451, t122457, t122460)
}
