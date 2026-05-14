//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1274/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1274<F: Float>(t140: F, t31976: F, t975: F, t32592: F, t32637: F, t110940: F, t9379: F, t110920: F, t110922: F, t110926: F, t110931: F, t110935: F, t110941: F, t110943: F, t110947: F, t15484: F, t2697: F, t9375: F) -> (F, F, F) {
    let t110950 = t140 * t975 * t31976;
    let t110952 = t32637 * t32592;
    let t110954 = t9379 * t110940;
    let t110956 = 0.41786499999999999999e-1 * t110920 - 0.24125000000000000001e-1 * t110922 - 0.120625e-1 * t110926 - 0.120625e-1 * t110931 + 0.69841875000000000003e-2 * t110935 - 0.69841875000000000003e-2 * t110941 - 0.24125000000000000001e-1 * t110943 + 0.72916666666666666668e-1 * t110947 - 0.2089325e-1 * t110950 + 0.56291666666666666668e-1 * t110952 - 0.62500000000000000002e-1 * t110954;
    let t110958 = t15484 * t9375 * t2697;
    (t110950, t110956, t110958)
}
