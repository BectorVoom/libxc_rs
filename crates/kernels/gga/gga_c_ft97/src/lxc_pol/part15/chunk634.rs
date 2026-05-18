//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 634/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk634<F: Float>(t173: F, t4483: F, t419: F, t4487: F, t375: F, t4496: F, t89: F, t4437: F, t1882: F, t4423: F, t358: F, t4495: F) -> (F, F, F, F, F, F) {
    let t15854 = t173 * t4483;
    let t15855 = t419 * t15854;
    let t15865 = t173 * t4487;
    let t15866 = t419 * t15865;
    let t15891 = t89 * t375 * t4496;
    let t15894 = t89 * t375 * t4437;
    let t15899 = t1882 * t4423;
    let t15901 = t4495 * t358;
    (t15855, t15866, t15891, t15894, t15899, t15901)
}
