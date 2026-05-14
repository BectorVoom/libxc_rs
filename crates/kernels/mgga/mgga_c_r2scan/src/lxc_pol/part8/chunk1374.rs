//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1374/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1374<F: Float>(t22134: F, t22139: F, t22143: F, t22148: F, t22152: F, t22161: F, t22169: F, t22173: F, t22176: F, t26556: F, t26561: F, t26563: F, t33540: F, t33542: F, t21699: F, t21702: F, t21705: F, t21709: F, t22183: F, t22186: F, t26564: F, t26567: F, t26576: F, t26585: F, t26588: F, t26590: F, t28665: F) -> (F, F) {
    let t33544 = -0.14035736694323150897e2 * t22134 + t22139 + 0.91082604192152556044e5 * t22143 + t22148 + t22152 + 0.42340699333333333333e-2 * t22161 + 0.3903689268108626343e0 * t22169 - 0.6858336e0 * t22173 + t26556 + t22176 - t26561 - t26563 - 4.0 * t33540 + 4.0 * t33542;
    let t33550 = t21699 + 0.4051561992e0 * t26564 + 0.127022098e-1 * t26567 + t26576 + t26585 - 0.16265371950452609763e-1 * t28665 + t21702 - t26588 - t21705 - t21709 + t26590 + 0.5848223622634646207e0 * t22183 - t22186;
    (t33544, t33550)
}
