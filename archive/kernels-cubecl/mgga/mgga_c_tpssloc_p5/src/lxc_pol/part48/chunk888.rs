//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 888/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk888<F: Float>(t225: F, t22624: F, t22622: F, t214: F, t3879: F, t1887: F, t22797: F, t2006: F, t3850: F, t268: F, t547: F, t6559: F) -> (F, F, F, F, F, F) {
    let t80699 = t22624 * t225;
    let t80704 = t22622 * t225;
    let t80707 = t214 * t3879;
    let t81159 = t22797 * t1887;
    let t81203 = t2006 * t3850;
    let t81228 = t6559 * t547 * t268;
    (t80699, t80704, t80707, t81159, t81203, t81228)
}
