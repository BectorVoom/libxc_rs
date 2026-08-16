//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2926;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta830<F: Float>(t17297: F, t2904: F, t952: F, t959: F, t300: F, t59774: F, t17304: F, t2940: F, t2929: F, t2932: F, t59975: F, t60037: F, t60039: F, t60041: F, t60044: F, t60047: F, t60050: F, t60053: F, t60056: F, t60354: F, t17938: F, t13663: F, t4483: F, t14259: F, t41825: F, t5774: F, t17566: F, t3213: F, t43637: F, t4700: F, t5950: F, t60359: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60391: F, t60394: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60915, t60917, t60919, t60923, t60924) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2926::<F>(t17297, t2904, t952, t959, t300, t59774, t17304, t2940, t2929, t2932, t59975, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60354);
        let (t60930, t60932, t60936, t60938, t60939) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927::<F>(t17938, t2940, t13663, t4483, t14259, t41825, t5774, t959, t17566, t3213, t43637, t4700, t5950, t60359, t60371, t60374, t60377, t60381, t60384, t60387, t60391, t60394);
    (t60915, t60917, t60919, t60923, t60924, t60930, t60932, t60936, t60938, t60939)
}
