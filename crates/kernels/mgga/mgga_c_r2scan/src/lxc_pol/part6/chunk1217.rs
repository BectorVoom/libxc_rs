//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1217/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1217<F: Float>(t1751: F, t5893: F, t124: F, t1762: F, t1818: F, t1983: F, t1949: F, t5215: F, t1771: F, t5416: F, t1987: F, t5916: F, t5216: F, t5967: F, t1835: F, t1946: F) -> (F, F, F, F, F, F, F) {
    let t22459 = t1751 * t5893;
    let t22464 = 0.76050639865105016044e2 * t1762 * t124 * t1818 * t1983;
    let t22467 = 0.17349730080482783747e0 * t1762 * t5215 * t1949;
    let t22468 = t1771 * t5416;
    let t22472 = 0.25685191006760837908e1 * t1762 * t5916 * t1987;
    let t22473 = t5967 * t5216;
    let t22478 = 0.77055573020282513724e1 * t1762 * t124 * t1835 * t1946;
    (t22459, t22464, t22467, t22468, t22472, t22473, t22478)
}
