//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1067/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1067<F: Float>(t14520: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t4496: F, t1568: F, t836: F, t231: F, t2783: F, t2782: F, t10519: F, t10524: F, t10943: F, t14498: F, t14502: F, t14506: F, t14507: F, t14511: F, t14512: F, t14518: F, t4366: F, t4494: F, t4504: F, t4514: F, t837: F) -> (F,) {
    let t14522 = 0.19514881078765566038e-1 * t2798 * t14520;
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    let t14535 = t1568 * t836;
    let t14537 = t2783 * t14535 * t231;
    let t14539 = 0.10975748638225852664e-1 * t2782 * t14537;
    let t14540 = 0.2601984143835408805e-1 * t10519 - 0.9757440539382783019e-2 * t10524 + t14498 + 0.13170898365871023197e1 * t4504 * t4494 * t10943 + t14506 + t14511 + 0.65049603595885220126e-3 * t14512 - 0.13170898365871023197e1 * t4514 * t14502 * t837 - t14518 - t14522 - 0.11565819519348392139e-2 * t14525 + 0.26341796731742046394e1 * t4504 * t14502 * t4366 + 0.26341796731742046394e1 * t4504 * t14507 * t4366 - 0.73171657588172351096e-2 * t14533 + t14539;
    (t14540,)
}
