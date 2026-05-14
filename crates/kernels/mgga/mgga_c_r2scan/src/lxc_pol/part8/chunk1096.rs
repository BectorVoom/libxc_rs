//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1096/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1096<F: Float>(t124: F, t1380: F, t1381: F, t1409: F, t1428: F, t1433: F, t1436: F, t1446: F, t1497: F, t1499: F, t1506: F, t18903: F, t18904: F, t19687: F, t19694: F, t19698: F, t19702: F, t19709: F, t19712: F, t377: F, t431: F, t439: F, t4754: F, t4758: F, t4762: F, t4768: F, t4772: F, t4816: F, t4817: F, t4818: F, t4860: F, t4862: F, t4874: F, t518: F, t625: F, t76: F) -> (F,) {
    let t19716 = -0.41096e0 * t625 * t4768 * t4874 - 0.14171548179536397724e3 * t625 * t377 * t4758 * t4762 - 0.68493333333333333332e-1 * t625 * t1428 * t4754 - 0.21309037037037037036e0 * t625 * t518 * t431 * t439 - 0.27397333333333333333e0 * t625 * t124 * t1433 * t1436 + 0.12842595503380418954e1 * t625 * t124 * t1380 * t1506 + 0.38527786510141256862e1 * t625 * t377 * t4816 * t4818 - 0.86748650402413918736e-1 * t625 * t124 * t1497 * t1499 + 0.13698666666666666666e0 * t625 * t4772 * t1446 - t19687 - 0.12304822629859687989e5 * t76 * t18903 * t18904 * t4862 + t19694 - t19698 - t19702 + 0.61524113149298439947e4 * t4860 * t1381 * t4862 * t1409 - t19709 + t19712 - 0.62337092780453269531e3 * t4817 * t1506 * t1409;
    (t19716,)
}
