//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1374/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1374<F: Float>(t29864: F, t9154: F, t2213: F, t238: F, t4265: F, t10937: F, t801: F, t1392: F, t242: F, t9027: F, t10944: F, t29851: F, t29853: F, t29855: F, t29857: F, t29860: F, t29862: F, t29865: F) -> (F, F, F, F, F, F) {
    let t29867 = t9154 * t29864;
    let t29870 = t238 * t2213 * t4265;
    let t29873 = t238 * t801 * t10937;
    let t29877 = t238 * t242 * t1392 * t9027;
    let t29880 = t238 * t801 * t10944;
    let t29882 = -F::new(0.258925e1) * t29851 - F::new(0.1294625e1) * t29853 - F::new(0.412621875e-1) * t29855 + F::new(0.16504875e0) * t29857 + F::new(0.16504875e0) * t29860 + F::new(0.82524375e-1) * t29862 + F::new(0.776775e1) * t29865 - F::new(0.16504875e0) * t29867 + F::new(0.27595e0) * t29870 - F::new(0.66228e0) * t29873 + F::new(0.49671e0) * t29877 - F::new(0.33114e0) * t29880;
    (t29867, t29870, t29873, t29877, t29880, t29882)
}
