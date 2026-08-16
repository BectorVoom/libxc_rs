//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1049/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1049<F: Float>(t30853: F, t30919: F, t3216: F, t8409: F, t11094: F, t8413: F, t1877: F, t193: F, t202: F, t2522: F, t30752: F, t30757: F, t30770: F, t6665: F, t6670: F, t776: F, t8366: F, t8370: F, t868: F, t870: F) -> (F, F, F, F) {
    let t30920 = t30853 + t30919;
    let t30924 = t8409 * t3216;
    let t30930 = t8413 * t11094;
    let t30952 = t193 * t202 * t30752 * t870 - t1877 * t30757 * t868 + F::cast_from(2.0_f64) * t1877 * t30770 * t868 - F::cast_from(2.0_f64) * t1877 * t6665 * t6670 + F::cast_from(3.0_f64) * t2522 * t776 * t8366 - F::cast_from(3.0_f64) * t2522 * t776 * t8370;
    (t30920, t30924, t30930, t30952)
}
