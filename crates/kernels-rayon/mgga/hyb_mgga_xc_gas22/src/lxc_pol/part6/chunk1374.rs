//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1374/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1374(t29864: f64, t9154: f64, t2213: f64, t238: f64, t4265: f64, t10937: f64, t801: f64, t1392: f64, t242: f64, t9027: f64, t10944: f64, t29851: f64, t29853: f64, t29855: f64, t29857: f64, t29860: f64, t29862: f64, t29865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29867 = t9154 * t29864;
    let t29870 = t238 * t2213 * t4265;
    let t29873 = t238 * t801 * t10937;
    let t29877 = t238 * t242 * t1392 * t9027;
    let t29880 = t238 * t801 * t10944;
    let t29882 = -0.258925e1_f64 * t29851 - 0.1294625e1_f64 * t29853 - 0.412621875e-1_f64 * t29855 + 0.16504875e0_f64 * t29857 + 0.16504875e0_f64 * t29860 + 0.82524375e-1_f64 * t29862 + 0.776775e1_f64 * t29865 - 0.16504875e0_f64 * t29867 + 0.27595e0_f64 * t29870 - 0.66228e0_f64 * t29873 + 0.49671e0_f64 * t29877 - 0.33114e0_f64 * t29880;
    (t29867, t29870, t29873, t29877, t29880, t29882)
}
