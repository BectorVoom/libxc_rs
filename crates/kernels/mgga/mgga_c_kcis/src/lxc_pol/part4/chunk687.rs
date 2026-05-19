//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 687/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk687<F: Float>(t1356: F, t3918: F, t3919: F, t3793: F, t3879: F, t3795: F, t3799: F, t3803: F, t3807: F, t3829: F, t3831: F, t3874: F, t3876: F, t3881: F, t3885: F, t3888: F, t3891: F) -> (F, F, F, F) {
    let t3921 = t3918 * t3919 * t1356;
    let t3926 = F::cast_from(0.40256666666666666667e0_f64) * t3793;
    let t3933 = F::new(0.137975e0) * t3879;
    let t3938 = -F::new(0.1294625e1) * t3829 + F::new(0.258925e1) * t3831 + t3926 + F::cast_from(0.20128333333333333334e0_f64) * t3795 - F::cast_from(0.20128333333333333333e0_f64) * t3799 + F::new(0.60385e0) * t3803 - F::new(0.301925e0) * t3807 + F::new(0.82524375e-1) * t3874 + F::new(0.16504875e0) * t3876 + t3933 + F::new(0.11038e0) * t3881 - F::new(0.27595e-1) * t3885 + F::new(0.16557e0) * t3888 - F::new(0.82785e-1) * t3891;
    (t3921, t3926, t3933, t3938)
}
