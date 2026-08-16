//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 634/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk634(t173: f64, t4483: f64, t419: f64, t4487: f64, t375: f64, t4496: f64, t89: f64, t4437: f64, t1882: f64, t4423: f64, t358: f64, t4495: f64) -> (f64, f64, f64, f64, f64, f64) {
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
