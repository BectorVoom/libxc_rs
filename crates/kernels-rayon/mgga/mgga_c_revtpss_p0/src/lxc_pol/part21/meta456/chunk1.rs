//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1988/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1988(t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t4496: f64, t1568: f64, t836: f64, t231: f64, t2783: f64, t2782: f64, t10519: f64, t10524: f64, t10943: f64, t14498: f64, t14502: f64, t14506: f64, t14507: f64, t14511: f64, t14512: f64, t14518: f64, t14522: f64, t4366: f64, t4494: f64, t4504: f64, t4514: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    let t14535 = t1568 * t836;
    let t14537 = t2783 * t14535 * t231;
    let t14539 = 0.10975748638225852664e-1_f64 * t2782 * t14537;
    let t14540 = 0.2601984143835408805e-1_f64 * t10519 - 0.9757440539382783019e-2_f64 * t10524 + t14498 + 0.13170898365871023197e1_f64 * t4504 * t4494 * t10943 + t14506 + t14511 + 0.65049603595885220126e-3_f64 * t14512 - 0.13170898365871023197e1_f64 * t4514 * t14502 * t837 - t14518 - t14522 - 0.11565819519348392139e-2_f64 * t14525 + 0.26341796731742046394e1_f64 * t4504 * t14502 * t4366 + 0.26341796731742046394e1_f64 * t4504 * t14507 * t4366 - 0.73171657588172351096e-2_f64 * t14533 + t14539;
    (t14523, t14524, t14535, t14537, t14540)
}
