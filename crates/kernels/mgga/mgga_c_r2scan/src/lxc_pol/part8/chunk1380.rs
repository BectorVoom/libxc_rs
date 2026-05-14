//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1380/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1380<F: Float>(t21829: F, t21832: F, t22375: F, t22379: F, t26741: F, t26743: F, t26745: F, t28774: F, t28776: F, t28778: F, t28780: F, t28783: F, t28785: F, t41: F, t725: F, t9904: F) -> (F, F) {
    let t33638 = t26741 + t21829 + t21832 - t26743 + t22375 + t22379 - 0.35089341735807877242e1 * t28774 - 0.31168546390226634766e3 * t28776 + 0.51947577317044391277e2 * t28778 + 0.10526802520742363173e2 * t28780 - 0.12154685976e1 * t28783 - 0.24309371952e1 * t28785 - t26745;
    let t33642 = t41 * t9904 * t725;
    (t33638, t33642)
}
