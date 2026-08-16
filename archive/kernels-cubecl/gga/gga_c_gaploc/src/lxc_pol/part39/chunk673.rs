//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 673/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk673<F: Float>(t9873: F, t3500: F, t7416: F, t10827: F, t2685: F, t2684: F, t2465: F, t2958: F, t2464: F, t787: F, t8788: F, t9824: F) -> (F, F, F, F, F) {
    let t10876 = F::cast_from(0.15976219147466979032e-1_f64) * t9873;
    let t10877 = t7416 * t3500;
    let t10878 = F::cast_from(0.19171462976960374838e0_f64) * t10877;
    let t10879 = t2685 * t10827;
    let t10880 = t2684 * t10879;
    let t10881 = F::cast_from(0.19171462976960374838e0_f64) * t10880;
    let t10882 = t2465 * t2958;
    let t10883 = t2464 * t10882;
    let t10884 = t2684 * t10883;
    let t10885 = F::cast_from(0.42603251059911944084e-1_f64) * t10884;
    let t10886 = t787 * t8788;
    let t10887 = t10886 * t9824;
    (t10876, t10878, t10881, t10885, t10887)
}
