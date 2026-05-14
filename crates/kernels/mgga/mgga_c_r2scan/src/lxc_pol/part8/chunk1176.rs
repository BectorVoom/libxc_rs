//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1176/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1176<F: Float>(t1762: F, t1767: F, t5664: F, t1771: F, t5418: F, t406: F, t5890: F, t424: F, t5714: F, t1751: F, t5893: F, t124: F, t1818: F, t1983: F, t1949: F, t5215: F) -> (F, F, F, F, F, F, F) {
    let t22449 = 0.64212977516902094772e0 * t1762 * t1767 * t5664;
    let t22450 = t1771 * t5418;
    let t22452 = t406 * t5890;
    let t22454 = t424 * t5714;
    let t22459 = t1751 * t5893;
    let t22464 = 0.76050639865105016044e2 * t1762 * t124 * t1818 * t1983;
    let t22467 = 0.17349730080482783747e0 * t1762 * t5215 * t1949;
    (t22449, t22450, t22452, t22454, t22459, t22464, t22467)
}
