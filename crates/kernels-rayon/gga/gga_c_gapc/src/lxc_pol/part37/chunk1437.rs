//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1437/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1437(t34176: f64, t34178: f64, t34193: f64, t34200: f64, t36849: f64, t36850: f64, t36851: f64, t36854: f64, t36855: f64, t36856: f64, t36857: f64, t34227: f64, t34230: f64, t36862: f64, t36863: f64, t36864: f64, t36865: f64, t36866: f64, t36867: f64, t36868: f64, t36869: f64, t36870: f64) -> (f64, f64) {
    let t38805 = t36849 + t36850 + t36851 + 0.36231816839129402172e-6_f64 * t34176 + 0.72463633678258804344e-6_f64 * t34178 - t36854 + t36855 + t36856 - t36857 + 0.7379489474826388889e-6_f64 * t34193 - 0.38527756621470067413e-7_f64 * t34200;
    let t38809 = t36862 + t36863 + t36864 + t36865 + t36866 - t36867 - t36868 - t36869 - t36870 + 0.84337022569444444446e-6_f64 * t34227 - 0.7379489474826388889e-6_f64 * t34230;
    (t38805, t38809)
}
