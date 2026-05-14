//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1025/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1025<F: Float>(t1748: F, t7731: F, t303: F, t1014: F, t8051: F, t26695: F, t4547: F, t4939: F, t27773: F, t922: F, t2894: F, t4781: F, t4947: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27940 = t1748 * t7731;
    let t27941 = t303 * t27940;
    let t27947 = t1014 * t8051;
    let t27949 = t26695 * t4547;
    let t27950 = t4939 * t27949;
    let t27953 = t27773 * t922;
    let t27954 = t2894 * t27953;
    let t27957 = t4781 * t922;
    let t27958 = t4947 * t27957;
    (t27940, t27941, t27947, t27949, t27950, t27953, t27954, t27957, t27958)
}
