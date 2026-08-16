//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 744/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk744<F: Float>(t4923: F, t904: F, t2601: F, t2608: F, t3746: F, t3795: F, t4828: F, t4832: F, t4836: F, t4848: F, t4855: F, t4861: F, t4863: F, t4867: F, t4870: F, t4873: F) -> (F, F) {
    let t4924 = t4923 * t904;
    let t4939 = -F::cast_from(0.1294625e1_f64) * t4848 + F::cast_from(0.258925e1_f64) * t4855 + t2601 + F::cast_from(0.20128333333333333334e0_f64) * t3746 - F::cast_from(0.20128333333333333333e0_f64) * t4828 + F::cast_from(0.60385e0_f64) * t4832 - F::cast_from(0.301925e0_f64) * t4836 + F::cast_from(0.82524375e-1_f64) * t4861 + F::cast_from(0.16504875e0_f64) * t4863 + t2608 + F::cast_from(0.11038e0_f64) * t3795 - F::cast_from(0.27595e-1_f64) * t4867 + F::cast_from(0.16557e0_f64) * t4870 - F::cast_from(0.82785e-1_f64) * t4873;
    (t4924, t4939)
}
