//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1104/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1104<F: Float>(t20624: F, t2288: F, t2310: F, t2313: F, t6561: F, t783: F, t2232: F, t230: F, t2235: F, t2180: F, t2233: F, t2187: F, t6578: F, t20688: F, t2186: F, t2306: F, t6669: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20714 = 280.0 / 81.0 * t20624;
    let t20730 = 1.0 / t2310 / t2288;
    let t20740 = t2310 * t2310;
    let t20741 = 1.0 / t20740;
    let t20743 = t2313 * t2313;
    let t20744 = 1.0 / t20743;
    let t20770 = 0.18467901234567901234e0 * t20624;
    let t20824 = t783 * t6561;
    let t20827 = t2232 * t2232;
    let t20829 = t230 / t20827;
    let t20831 = t2235 * t2235;
    let t20832 = 1.0 / t20831;
    let t20838 = t2180 * t2233;
    let t20843 = t2180 * t2187;
    let t20846 = t783 * t6578;
    let t20853 = 0.31003950617283950618e1 * t20624;
    let t20867 = 0.13388493827160493828e1 * t20688;
    let t20895 = t230 / t2232 / t2186;
    let t20904 = 0.96141975308641975307e-1 * t20624;
    let t20934 = t2306 * t6669;
    (t20714, t20730, t20741, t20744, t20770, t20824, t20829, t20832, t20838, t20843, t20846, t20853, t20867, t20895, t20904, t20934)
}
