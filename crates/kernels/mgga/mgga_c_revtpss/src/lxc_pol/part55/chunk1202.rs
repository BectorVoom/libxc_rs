//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1202/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1202<F: Float>(t1962: F, t4537: F, t4343: F, t119765: F, t119779: F, t126043: F, t126049: F, t126052: F, t126055: F, t126062: F, t126065: F, t126068: F, t126072: F, t126076: F) -> (F, F, F) {
    let t127593 = t1962 * t4537;
    let t127596 = t1962 * t4343;
    let t127615 = -F::new(0.34708173928447610099e-2) * t126043 - t119765 + F::new(0.225875734067843736e-2) * t126049 - F::new(0.29749863367240808656e-2) * t126052 - F::new(0.22312397525430606492e-2) * t126055 - t119779 - F::new(0.22312397525430606492e-2) * t126062 - F::new(0.29749863367240808656e-2) * t126065 + F::new(0.7437465841810202164e-3) * t126068 - F::new(0.14874931683620404328e-2) * t126072 - F::new(0.14874931683620404328e-2) * t126076;
    (t127593, t127596, t127615)
}
