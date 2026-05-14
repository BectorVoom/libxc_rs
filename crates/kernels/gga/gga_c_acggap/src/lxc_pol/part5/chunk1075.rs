//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1075/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1075<F: Float>(t4795: F, t5645: F, t721: F, t5651: F, t13728: F, t5656: F, t3111: F, t5660: F, t1072: F, t13703: F, t1713: F, t3126: F, t1060: F, t355: F, t5506: F, t1734: F, t3124: F) -> (F, F, F, F, F, F, F) {
    let t21740 = t4795 * t5645 * t721;
    let t21743 = t4795 * t5651 * t721;
    let t21745 = t13728 * t5656;
    let t21747 = t3111 * t5660;
    let t21751 = t13703 * t1072 * t1713 * t3126;
    let t21755 = t1060 * t355 * t5506 * t721;
    let t21759 = t3124 * t1072 * t1734 * t3126;
    (t21740, t21743, t21745, t21747, t21751, t21755, t21759)
}
