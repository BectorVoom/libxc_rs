//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2926;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta830(t17297: f64, t2904: f64, t952: f64, t959: f64, t300: f64, t59774: f64, t17304: f64, t2940: f64, t2929: f64, t2932: f64, t59975: f64, t60037: f64, t60039: f64, t60041: f64, t60044: f64, t60047: f64, t60050: f64, t60053: f64, t60056: f64, t60354: f64, t17938: f64, t13663: f64, t4483: f64, t14259: f64, t41825: f64, t5774: f64, t17566: f64, t3213: f64, t43637: f64, t4700: f64, t5950: f64, t60359: f64, t60371: f64, t60374: f64, t60377: f64, t60381: f64, t60384: f64, t60387: f64, t60391: f64, t60394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60915, t60917, t60919, t60923, t60924) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2926(t17297, t2904, t952, t959, t300, t59774, t17304, t2940, t2929, t2932, t59975, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60354);
        let (t60930, t60932, t60936, t60938, t60939) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927(t17938, t2940, t13663, t4483, t14259, t41825, t5774, t959, t17566, t3213, t43637, t4700, t5950, t60359, t60371, t60374, t60377, t60381, t60384, t60387, t60391, t60394);
    (t60915, t60917, t60919, t60923, t60924, t60930, t60932, t60936, t60938, t60939)
}
