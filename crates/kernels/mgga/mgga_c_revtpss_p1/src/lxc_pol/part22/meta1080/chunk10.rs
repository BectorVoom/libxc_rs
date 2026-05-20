//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3892/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3892<F: Float>(t13739: F, t13743: F, t13746: F, t1424: F, t22387: F, t22395: F, t4071: F, t4131: F, t47561: F, t47568: F, t47793: F, t47794: F, t47947: F, t47952: F, t49468: F, t49472: F, t49474: F, t49476: F, t49480: F, t5715: F, t6895: F, t74757: F, t74763: F, t74770: F, t74782: F, t9657: F) -> F {
    let t74786 = F::cast_from(0.52039682876708176102e-1_f64) * t47947 + F::cast_from(0.29268663035268940438e-1_f64) * t47952 - F::cast_from(0.15805078039045227836e2_f64) * t47793 * t47794 * t13746 + t47561 - F::cast_from(0.34146773541147097178e-1_f64) * t49468 - F::cast_from(0.13009920719177044025e-2_f64) * t74757 + F::cast_from(0.26341796731742046394e1_f64) * t5715 * t13739 + F::cast_from(0.39029762157531132076e-1_f64) * t49472 - F::cast_from(0.22089088168956307394e-3_f64) * t49474 + F::cast_from(0.78059524315062264149e-1_f64) * t74763 - F::cast_from(0.79025390195226139182e1_f64) * t5715 * t13743 - F::cast_from(0.29268663035268940438e-1_f64) * t49476 + F::cast_from(0.23131639038696784277e-2_f64) * t74770 - F::cast_from(0.2601984143835408805e-2_f64) * t49480 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t9657 * t6895 * t4131 + F::cast_from(0.52683593463484092788e1_f64) * t4071 * t22395 + F::cast_from(0.22089088168956307394e-3_f64) * t47568 + F::cast_from(0.39029762157531132075e-1_f64) * t74782 - F::cast_from(0.13170898365871023197e1_f64) * t4071 * t22387;
    t74786
}
