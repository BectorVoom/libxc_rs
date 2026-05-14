//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 846/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk846<F: Float>(t14026: F, t962: F, t971: F, t1680: F, t2939: F, t2986: F, t2980: F, t4722: F, t1679: F, t9770: F, t9768: F, t4690: F, t9804: F, t4723: F, t9655: F, t4719: F, t949: F) -> (F, F, F, F, F, F, F) {
    let t14028 = t962 * t14026 * t971;
    let t14033 = t1680 * t2939;
    let t14035 = 6.0 * t2986 * t14033;
    let t14036 = t4722 * t2980;
    let t14038 = 0.16081824322151104822e2 * t2986 * t14036;
    let t14039 = t1679 * t9770;
    let t14040 = t14039 * t2939;
    let t14042 = 0.51725014705706168417e3 * t9768 * t14040;
    let t14044 = 4.0 * t9804 * t4690;
    let t14046 = 0.32163648644302209644e2 * t9655 * t4723;
    let t14047 = t4719 * t949;
    (t14028, t14035, t14038, t14042, t14044, t14046, t14047)
}
