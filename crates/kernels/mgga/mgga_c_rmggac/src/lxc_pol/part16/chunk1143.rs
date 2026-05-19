//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1143/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1143<F: Float>(t10387: F, t1632: F, t2471: F, t42042: F, t42057: F, t43987: F, t47800: F, t47802: F, t47804: F, t47809: F, t47814: F, t47816: F, t47821: F, t47826: F, t47831: F, t47833: F, t47835: F, t49327: F, t4985: F, t5019: F, t739: F, t903: F, t9302: F) -> F {
    let t49709 = -F::cast_from(0.23948483403727617128e0_f64) * t5019 * t10387 + F::cast_from(0.12195059916630011325e-2_f64) * t42042 - F::cast_from(0.40911992481368012596e-1_f64) * t47800 - F::cast_from(0.16364796992547205038e0_f64) * t47802 - F::cast_from(0.40911992481368012596e-1_f64) * t47804 - F::cast_from(0.3405167991463827152e-4_f64) * t47809 + F::cast_from(0.1702583995731913576e-4_f64) * t47814 + F::cast_from(0.212822999466489197e-4_f64) * t47816 + t43987 + F::cast_from(0.39726959900411316773e-4_f64) * t47821 + F::cast_from(0.212822999466489197e-4_f64) * t47826 + F::cast_from(0.17562221162733585894e1_f64) * t42057 - F::cast_from(0.11974241701863808564e0_f64) * t47831 - F::cast_from(0.14369090042236570277e1_f64) * t47833 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2471 * t1632 + F::cast_from(0.31931311204970156171e0_f64) * t47835 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t49327 + F::cast_from(0.11974241701863808564e0_f64) * t4985 * t9302;
    t49709
}
