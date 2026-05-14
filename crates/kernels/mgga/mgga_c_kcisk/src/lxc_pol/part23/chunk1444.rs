//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1444/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1444<F: Float>(t5579: F, t9406: F, t111224: F, t111472: F, t111507: F, t111509: F, t111512: F, t111515: F, t111518: F, t111564: F, t1152: F, t1156: F, t2070: F, t2709: F, t294: F, t33338: F, t33980: F, t33990: F, t4569: F, t9408: F) -> (F,) {
    let t116014 = 2.0 * t5579 * t9406;
    let t116023 = -t111224 + t1152 * t33990 / 8.0 + t111472 + t116014 + t111564 - t111507 + t111509 + t111512 - t111515 + t111518 - t2709 * t2070 * t4569 / 16.0 + t9408 * t33338 / 8.0 - t294 * t1156 * t33980 / 8.0;
    (t116023,)
}
