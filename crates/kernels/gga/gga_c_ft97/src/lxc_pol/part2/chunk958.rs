//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 958/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk958<F: Float>(t292: F, t14769: F, t14887: F, t799: F, t27: F, t89: F, t1213: F, t1636: F, t668: F, t848: F, t2999: F, t375: F, t4130: F) -> (F, F, F, F, F) {
    let t293 = F::new(0.1e-59) < t292;
    let t14889 = piecewise3::<f64>(t293, t14769 + t14887, F::new(0.0));
    let t14890 = t799 * t14889;
    let t14892 = t89 * t27 * t14890;
    let t14895 = t89 * t1636 * t1213;
    let t14897 = t848 * t668;
    let t14899 = t89 * t2999 * t14897;
    let t14902 = t89 * t375 * t4130;
    (t14889, t14892, t14895, t14899, t14902)
}
