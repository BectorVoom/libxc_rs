//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 429/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk429<F: Float>(t1101: F, t2694: F, t2697: F, t2683: F, t2687: F, t2690: F) -> (F, F) {
    let t2698 = t1101 * t2694 * t2697;
    let t2701 = -0.10416666666666666667e-1 * t2698 + 0.69644166666666666665e-2 * t2683;
    let t2705 = 0.1875e0 * t2687 - 0.809375e-1 * t2690;
    (t2701, t2705)
}
