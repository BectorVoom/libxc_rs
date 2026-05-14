//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1385/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1385<F: Float>(t21976: F, t21985: F, t21988: F, t22000: F, t22496: F, t22500: F, t22505: F, t22512: F, t22521: F, t22523: F, t22528: F, t26835: F, t26839: F, t33697: F, t22003: F, t22010: F, t22012: F, t22534: F, t22537: F, t22542: F, t22546: F, t22550: F, t22554: F, t22557: F, t22560: F, t22574: F, t26849: F, t28882: F) -> (F, F) {
    let t33700 = -t22496 - t22500 - 0.11696447245269292414e1 * t22505 - t21976 - t21985 + 0.80040858019733333332e-2 * t26835 + t26839 - t21988 - 0.571528e-1 * t33697 + t22000 - t22512 + t22521 - t22523 + 0.16206247968e1 * t22528;
    let t33704 = -t22534 + t22537 + t22542 + t22546 + t22550 - t22554 + 0.1800919305444e-1 * t26849 + 0.254044196e-2 * t28882 + t22003 + t22557 + t22560 - t22574 - t22010 + t22012;
    (t33700, t33704)
}
