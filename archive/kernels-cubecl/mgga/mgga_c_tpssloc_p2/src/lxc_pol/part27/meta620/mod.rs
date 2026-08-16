//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2098;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta620<F: Float>(t1920: F, t2966: F, t6699: F, t1921: F, t82457: F, t23314: F, t23384: F, t6707: F, t82632: F, t23734: F, t3216: F, t11094: F, t6818: F, t1958: F, t43637: F, t1081: F, t2752: F, t1864: F, t2241: F, t1863: F, t608: F, t9231: F, t22550: F, t6505: F, t645: F, t6509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83444, t83453, t83457, t83459, t83468, t83472) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2098::<F>(t1920, t2966, t6699, t1921, t82457, t23314, t23384, t6707, t82632, t23734, t3216, t11094, t6818);
        let (t83479, t83555, t83719, t83722, t83725, t83728) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2099::<F>(t1958, t43637, t1081, t2752, t1864, t2241, t1863, t608, t9231, t22550, t6505, t645, t6509);
    (t83444, t83453, t83457, t83459, t83468, t83472, t83479, t83555, t83719, t83722, t83725, t83728)
}
