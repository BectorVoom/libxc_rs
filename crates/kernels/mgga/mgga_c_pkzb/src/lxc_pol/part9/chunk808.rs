//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 808/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk808<F: Float>(t1901: F, t5737: F, t5776: F, t5519: F, t5557: F, t5513: F, t5516: F, t5522: F, t5525: F, t5539: F, t5541: F, t5548: F, t5551: F, t5553: F, t5560: F, t5563: F, t5566: F, t5570: F, t5574: F) -> (F, F, F, F, F) {
    let t5777 = t5737 * t1901;
    let t5779 = F::new(0.96491876992155210402e2) * t5776 * t5777;
    let t5783 = F::new(0.93011851851851851854e0) * t5519;
    let t5790 = F::new(0.36514074074074074075e0) * t5557;
    let t5796 = F::new(0.142419375e1) * t5513 - F::new(0.28483875e1) * t5516 + F::new(0.1898925e1) * t5541 - t5783 + F::new(0.11958666666666666667e1) * t5522 - F::new(0.89690000000000000001e0) * t5525 + F::new(0.8969e0) * t5539 - F::new(0.76790625e-1) * t5548 + F::new(0.46074375e0) * t5551 + F::new(0.3071625e0) * t5553 - t5790 + F::new(0.82156666666666666666e0) * t5560 - F::new(0.49293999999999999999e0) * t5563 - F::new(0.49293999999999999999e0) * t5566 + F::new(0.73941e0) * t5570 + F::new(0.24647e0) * t5574;
    (t5777, t5779, t5783, t5790, t5796)
}
