//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2284/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2284<F: Float>(t1307: F, t1385: F, t22635: F, t26331: F, t26337: F, t26216: F, t81159: F, t26210: F, t6897: F, t794: F, t1377: F, t5187: F) -> (F, F, F, F, F) {
    let t90506 = t1307 * t1385;
    let t90509 = t26331 * t22635 * t26337 * t90506;
    let t90511 = t81159 * t26216;
    let t90512 = F::cast_from(0.76763589786250567036e-1_f64) * t90511;
    let t90514 = t6897 * t794 * t26210;
    let t90515 = F::cast_from(0.82246703342411321824e-2_f64) * t90514;
    let t90516 = t1377 * t5187;
    (t90506, t90509, t90512, t90515, t90516)
}
