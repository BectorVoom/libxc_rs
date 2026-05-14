//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1188/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1188<F: Float>(t34669: F, t34641: F, t34647: F, t34651: F, t34654: F, t34658: F, t37020: F, t37022: F, t37027: F, t37028: F, t37029: F, t34673: F, t34676: F, t34679: F, t34682: F, t34686: F) -> (F, F, F, F, F, F) {
    let t37030 = 0.4637672555408563478e-4 * t34669;
    let t37031 = -t37020 - 0.71958020936198887258e-7 * t34641 + t37022 + 0.95956020918421216158e-7 * t34647 + 0.98332751566569010434e-8 * t34651 + 0.49166375783284505217e-8 * t34654 - 0.65555167711046006956e-8 * t34658 - t37027 - t37028 + t37029 + t37030;
    let t37032 = 0.69504740211613770836e-3 * t34673;
    let t37033 = 0.9275345110817126956e-4 * t34676;
    let t37034 = 0.69504740211613770836e-3 * t34679;
    let t37035 = 0.35265860507710533408e-5 * t34682;
    let t37036 = 0.4637672555408563478e-4 * t34686;
    (t37031, t37032, t37033, t37034, t37035, t37036)
}
