//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2059;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta646(t1409: f64, t1937: f64, t6722: f64, t14501: f64, t23419: f64, t1015: f64, t23472: f64, t25678: f64, t7554: f64, t82632: f64, t225: f64, t25820: f64, t23384: f64, t25827: f64, t25436: f64, t23328: f64, t23394: f64, t1054: f64, t4693: f64, t13783: f64, t1926: f64, t221: f64, t25432: f64, t25806: f64, t6680: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88692, t88704, t88723, t88731, t88744) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2059(t1409, t1937, t6722, t14501, t23419, t1015, t23472, t25678, t7554, t82632, t225, t25820);
        let (t88753, t88758, t88772, t88804, t88810, t88812, t88845) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060(t23384, t25827, t25436, t23328, t23394, t1054, t4693, t13783, t1926, t221, t25432, t25806, t6680);
    (t88692, t88704, t88723, t88731, t88744, t88753, t88758, t88772, t88804, t88810, t88812, t88845)
}
