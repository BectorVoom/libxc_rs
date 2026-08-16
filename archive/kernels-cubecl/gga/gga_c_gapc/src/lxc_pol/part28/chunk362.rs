//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 362/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk362<F: Float>(t122: F, t136: F, t653: F, t116: F, t1033: F, t190: F, t1037: F, t5: F, t198: F, t186: F, t187: F, t128: F, t195: F) -> (F, F, F, F, F, F, F, F) {
    let t1620 = t136 * t122;
    let t1621 = t1620 * t653;
    let t1622 = t116 * t1621;
    let t1623 = t190 * t1033;
    let t1625 = t1037 * t5;
    let t1626 = t1623 * t198 * t1625;
    let t1629 = t136 * t186;
    let t1630 = t187 * t187;
    let t1631 = F::cast_from(1.0_f64) / t1630;
    let t1632 = t1629 * t1631;
    let t1633 = t116 * t1632;
    let t1636 = t128 * t195;
    (t1622, t1623, t1625, t1626, t1629, t1631, t1633, t1636)
}
