//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 991/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk991<F: Float>(t1666: F, t2937: F, t2940: F, t4682: F, t930: F, t951: F, t2981: F, t4685: F, t2985: F, t2989: F, t1680: F, t9650: F, t2933: F, t4719: F, t1670: F, t9752: F) -> (F, F, F, F, F, F, F) {
    let t13864 = t1666 * t2937;
    let t13866 = 2.0 * t13864 * t2940;
    let t13867 = t4682 * t930;
    let t13869 = 2.0 * t13867 * t951;
    let t13871 = 1.0 * t4685 * t2981;
    let t13872 = t1666 * t2985;
    let t13874 = 0.16081824322151104822e2 * t13872 * t2989;
    let t13876 = 1.0 * t9650 * t1680;
    let t13878 = 2.0 * t2933 * t4719;
    let t13880 = t9752 * t1670;
    (t13866, t13869, t13871, t13874, t13876, t13878, t13880)
}
