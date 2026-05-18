//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 810/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk810<F: Float>(t1457: F, t7245: F, t1445: F, t7259: F, t2089: F, t2530: F, t723: F, t1645: F, t1885: F, t7499: F, t7227: F, t1880: F, t733: F) -> (F, F, F, F, F, F, F, F) {
    let t7751 = t1457 * t7245;
    let t7756 = t1445 * t7259;
    let t7759 = t1445 * t7245;
    let t7764 = t2089 * t2530;
    let t7765 = t7764 * t723;
    let t7766 = t1445 * t7765;
    let t7769 = t1645 * t1885;
    let t7772 = t1457 * t7499;
    let t7775 = t1457 * t7227;
    let t7778 = t733 * t1880;
    (t7751, t7756, t7759, t7766, t7769, t7772, t7775, t7778)
}
