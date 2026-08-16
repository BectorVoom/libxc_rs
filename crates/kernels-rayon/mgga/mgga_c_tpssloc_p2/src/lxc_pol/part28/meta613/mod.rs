//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta613(t26421: f64, t26446: f64, t3734: f64, t90591: f64, t22751: f64, t26389: f64, t1992: f64, t22897: f64, t3792: f64, t90870: f64, t26467: f64, t6914: f64, t26426: f64, t81046: f64, t22690: f64, t7732: f64, t81195: f64, t16413: f64, t1985: f64, t1998: f64, t214: f64, t16248: f64, t22833: f64, t16383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91052, t91064, t91074, t91076) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927(t26421, t26446, t3734, t90591, t22751, t26389, t1992, t22897, t3792, t90870, t26467, t6914);
        let (t91078, t91081, t91091, t91094, t91096) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928(t26426, t81046, t22690, t7732, t81195, t16413, t1985, t1998, t214, t16248, t22833, t16383);
    (t91052, t91064, t91074, t91076, t91078, t91081, t91091, t91094, t91096)
}
