//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1113/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1113<F: Float>(t7411: F, t9216: F, t7483: F, t9219: F, t20911: F, t9222: F, t10892: F, t5776: F, t683: F, t3622: F, t7560: F, t20637: F, t2852: F, t30231: F, t3626: F, t2875: F, t9242: F) -> (F, F, F, F, F, F, F, F) {
    let t30261 = 18.0 * t7411 * t9216;
    let t30263 = 12.0 * t7483 * t9219;
    let t30265 = 0.2894756309764656312e3 * t20911 * t9222;
    let t30268 = 24.0 * t5776 * t10892 * t683;
    let t30270 = 0.17544670867903938621e1 * t7560 * t3622;
    let t30273 = 0.31168546390226634766e3 * t20637 * t2852 * t30231;
    let t30275 = 0.51947577317044391276e2 * t7560 * t3626;
    let t30277 = 0.51947577317044391276e2 * t9242 * t2875;
    (t30261, t30263, t30265, t30268, t30270, t30273, t30275, t30277)
}
