//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2643/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2643(t1256: f64, t19267: f64, t193: f64, t27843: f64, t336: f64, t4700: f64, t5091: f64, t66897: f64, t72104: f64, t72106: f64, t72138: f64, t72201: f64, t72203: f64, t72207: f64, t72209: f64, t72211: f64, t72213: f64, t73852: f64, t73885: f64, t73919: f64) -> f64 {
    let t73931 = -t72104 - t72106 + t193 * t336 * (t72138 + t73852 + t73885 + t73919) * t1256 - t72201 - t72203 + 6.0_f64 * t4700 * t66897 * t27843 + t72207 - t72209 + t72211 - t72213 - 3.0_f64 * t4700 * t19267 * t5091;
    t73931
}
