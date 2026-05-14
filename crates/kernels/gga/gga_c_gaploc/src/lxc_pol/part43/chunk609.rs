//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 609/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk609<F: Float>(t10741: F, t10745: F, t10747: F, t10751: F, t10754: F, t10757: F, t10759: F, t10762: F, t10765: F, t10767: F, t10769: F, t10772: F, t10775: F, t10788: F, t12318: F, t1841: F) -> (F,) {
    let t12321 = -t10741 - t10745 + t10747 + t10751 + t10754 - t10757 - t10759 - t10762 - t10765 + t10767 + t10769 + t10772 + t10775 + t10788 + 0.85450291446024714263e-3 * t1841 * t12318;
    (t12321,)
}
